mod media;
mod rich_text;
mod utils;

use atprotolib_rs::{
    api_calls::{ApiAuthBearerToken, ApiAuthConfig, ApiAuthConfigData},
    types::{
        app_bsky,
        com_atproto::{self, server::CreateSessionRequest}
    }
};
use diesel::r2d2::{ConnectionManager, Pool};
use fediproto_sync_db::{
    models::{NewMastodonPost, NewSyncedPostBlueSkyData},
    AnyConnection
};
use fediproto_sync_lib::error::{FediProtoSyncError, FediProtoSyncErrorKind};

#[allow(unused_imports)]
pub use self::{
    media::{BlueSkyPostSyncMedia, MAX_IMAGE_SIZE, MAX_VIDEO_SIZE, MAX_VIDEO_DURATION},
    rich_text::BlueSkyPostSyncRichText,
    utils::BlueSkyPostSyncUtils
};
use crate::FediProtoSyncConfig;

/// Holds the authentication information for a Bluesky session.
#[derive(Debug, Clone)]
pub struct BlueSkyAuthentication {
    /// The hostname of the BlueSky/ATProto PDS.
    pub host_name: String,

    /// The API authentication configuration for the session.
    pub auth_config: ApiAuthConfig,

    /// The session information for the authenticated BlueSky session.
    pub session: com_atproto::server::CreateSessionResponse
}

impl BlueSkyAuthentication {
    /// Creates a new `BlueSkyAuthentication` instance.
    ///
    /// ## Arguments
    ///
    /// * `config` - The environment variables for the FediProto Sync
    ///   application.
    /// * `client` - The reqwest client to use for the API request.
    pub async fn new(
        config: &FediProtoSyncConfig,
        client: reqwest::Client
    ) -> Result<Self, FediProtoSyncError> {
        let config = config.clone();

        let initial_auth_config = ApiAuthConfig {
            data: ApiAuthConfigData::None
        };

        let bsky_create_session = com_atproto::server::create_session(
            &config.bluesky_pds_server,
            client,
            &initial_auth_config,
            CreateSessionRequest {
                identifier: config.bluesky_handle.clone(),
                password: config.bluesky_app_password.clone(),
                auth_factor_token: None
            }
        )
        .await
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to create Bluesky session.",
                FediProtoSyncErrorKind::AuthenticationError,
                e
            )
        })?;

        let bsky_auth_config = ApiAuthConfig {
            data: ApiAuthConfigData::BearerToken(ApiAuthBearerToken {
                token: bsky_create_session.access_jwt.clone()
            })
        };

        Ok(Self {
            host_name: config.bluesky_pds_server.clone(),
            auth_config: bsky_auth_config,
            session: bsky_create_session
        })
    }

    /// Refresh the Bluesky session token.
    ///
    /// ## Arguments
    ///
    /// * `client` - The reqwest client to use for the API request.
    pub async fn refresh_session_token(
        &mut self,
        client: reqwest::Client
    ) -> Result<(), FediProtoSyncError> {
        let refresh_auth_config = ApiAuthConfig {
            data: ApiAuthConfigData::BearerToken(ApiAuthBearerToken {
                token: self.session.refresh_jwt.clone()
            })
        };

        let bsky_refresh_session =
            com_atproto::server::refresh_session(&self.host_name, client, &refresh_auth_config)
                .await
                .map_err(|e| {
                    FediProtoSyncError::with_source(
                        "Failed to refresh Bluesky session.",
                        FediProtoSyncErrorKind::AuthenticationError,
                        e
                    )
                })?;

        let bsky_auth_config = ApiAuthConfig {
            data: ApiAuthConfigData::BearerToken(ApiAuthBearerToken {
                token: bsky_refresh_session.access_jwt.clone()
            })
        };

        self.auth_config = bsky_auth_config;
        self.session = bsky_refresh_session;

        Ok(())
    }
}

/// Struct to hold the data and logic for syncing a Mastodon post to BlueSky.
pub struct BlueSkyPostSync {
    /// The environment variables for the FediProto Sync application.
    pub config: FediProtoSyncConfig,

    /// The authentication session for BlueSky.
    pub bsky_auth: BlueSkyAuthentication,

    /// The database connection for the FediProto Sync application.
    pub db_connection_pool: Pool<ConnectionManager<AnyConnection>>,

    /// The Mastodon account that posted the status.
    pub mastodon_account: megalodon::entities::account::Account,

    /// The Mastodon status.
    pub mastodon_status: megalodon::entities::Status,

    /// The post generated from the Mastodon status to sync to BlueSky.
    pub post_item: app_bsky::feed::Post
}

impl BlueSkyPostSync {
    /// Sync a Mastodon post to Bluesky.
    pub async fn sync_post(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let db_connection = &mut self.db_connection_pool.get()?;

        // -- Generate a BlueSky post item from the Mastodon status. --
        self.generate_post_item().await?;

        // -- Check if the post is a reply to another post in a thread. --
        let mut previous_post_id = None;

        // If the post has a "in_reply_to_id" field and the "in_reply_to_account_id"
        // field is the same as the account ID of the account that posted the status,
        // then it is potentially a reply to another post in a thread.
        if self.mastodon_status.in_reply_to_id.is_some()
            && self
                .mastodon_status
                .clone()
                .in_reply_to_account_id
                .unwrap_or_else(|| "".to_string())
                == self.mastodon_account.id.clone()
        {
            let in_reply_to_id = self.mastodon_status.in_reply_to_id.clone().unwrap();

            match fediproto_sync_db::operations::check_synced_mastodon_post_exists(
                db_connection,
                &in_reply_to_id
            )? {
                true => {}

                false => {
                    return Err(Box::new(FediProtoSyncError::new(
                        format!("Previous post '{}' not found in database.", &in_reply_to_id)
                            .as_str(),
                        FediProtoSyncErrorKind::DatabaseQueryError
                    )));
                }
            };

            // Resolve the previous post in the thread and resolve it's synced post data.
            let previous_mastodon_post =
                fediproto_sync_db::operations::get_synced_mastodon_post_by_id(
                    db_connection,
                    &in_reply_to_id
                )?;
            let previous_synced_post =
                fediproto_sync_db::operations::get_bluesky_data_by_mastodon_post_id(
                    db_connection,
                    &in_reply_to_id
                )?;

            // If the previous post has a root post, resolve it's synced post data.
            let previous_synced_post_root = match previous_mastodon_post.root_mastodon_post_id {
                Some(root_mastodon_post_id) => {
                    // Set the previous post ID to the root post ID retrieved.
                    previous_post_id = Some(root_mastodon_post_id.clone());

                    fediproto_sync_db::operations::get_bluesky_data_by_mastodon_post_id(
                        db_connection,
                        &root_mastodon_post_id
                    )?
                }
                None => {
                    // Set the previous post ID to the previous post ID.
                    previous_post_id = Some(in_reply_to_id.clone());

                    previous_synced_post.clone()
                }
            };

            // Set the reply reference for the post item.
            self.post_item.reply_ref = Some(app_bsky::feed::PostReplyRef {
                root: com_atproto::repo::StrongRef {
                    cid: previous_synced_post_root.bsky_post_cid.clone(),
                    uri: previous_synced_post_root.bsky_post_uri.clone()
                },
                parent: com_atproto::repo::StrongRef {
                    cid: previous_synced_post.bsky_post_cid.clone(),
                    uri: previous_synced_post.bsky_post_uri.clone()
                }
            });
        }

        // -- Send the post item to BlueSky through the 'com.atproto.repo.applyWrites'
        // API. --
        let apply_write_request = com_atproto::repo::ApplyWritesRequest {
            repo: self.bsky_auth.session.did.clone(),
            validate: true,
            writes: vec![com_atproto::repo::ApplyWritesRequestWrites::Create(
                com_atproto::repo::Create::new(
                    "app.bsky.feed.post",
                    com_atproto::repo::ApplyWritesValue::Post(self.post_item.clone())
                )
            )],
            swap_commit: None
        };

        let apply_write_client = crate::core::create_http_client(&self.config)?;

        let apply_write_result = com_atproto::repo::apply_writes(
            &self.bsky_auth.host_name,
            apply_write_client,
            &self.bsky_auth.auth_config,
            apply_write_request
        )
        .await;

        // -- Handle the response from the 'com.atproto.repo.applyWrites' API. --
        match apply_write_result {
            Ok(result) => {
                // If no HTTP errors occurred, get the results from the response.
                // We need the CID and URI of the post that was created from it.
                let post_result = match result.results.first().unwrap() {
                    com_atproto::repo::ApplyWritesResponseResults::CreateResult(create_result) => {
                        create_result
                    }

                    _ => panic!("Unexpected response from Bluesky")
                };

                let new_mastodon_post = NewMastodonPost::new(
                    &self.mastodon_status,
                    Some(post_result.cid.clone()),
                    previous_post_id
                );

                let new_synced_post = NewSyncedPostBlueSkyData::new(
                    &self.mastodon_status.id,
                    &post_result.cid,
                    &post_result.uri
                );

                // Insert the synced Mastodon post into the database for future tracking.
                fediproto_sync_db::operations::insert_new_synced_mastodon_post(
                    db_connection,
                    &new_mastodon_post
                )?;

                // Insert the synced BlueSky post into the database for future tracking.
                fediproto_sync_db::operations::insert_new_bluesky_data_for_synced_mastodon_post(
                    db_connection,
                    &new_synced_post
                )?;

                tracing::info!("Synced post '{}' to BlueSky.", &self.mastodon_status.id);

                Ok(())
            }

            Err(error) => {
                // If an error occurred, log the error and return it.
                tracing::error!(
                    "Error syncing post '{}': {}",
                    &self.mastodon_status.id,
                    error
                );
                Err(error)
            }
        }
    }

    /// Generate a Bluesky post item from a Mastodon status.
    pub async fn generate_post_item(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // -- Parse the Mastodon status for post content and metadata. --
        let mut parsed_status =
            crate::mastodon::ParsedMastodonPost::from_mastodon_status(&self.mastodon_status)?;

        // -- Create a Bluesky post item from the parsed Mastodon status. --

        // Truncate the post content to fit within the 300 character limit of Bluesky.
        parsed_status.truncate_post_content()?;

        // Create the post item with the parsed post content and metadata.
        self.post_item = app_bsky::feed::Post::new(
            &parsed_status.stripped_html,
            parsed_status.mastodon_status.created_at,
            None
        );

        // Check if the post has any media attachments and upload them to Bluesky.
        if parsed_status.mastodon_status.media_attachments.len() > 0 {
            tracing::info!(
                "Found '{}' media attachments in post '{}'",
                parsed_status.mastodon_status.media_attachments.len(),
                parsed_status.mastodon_status.id
            );

            let first_media_attachment = parsed_status.mastodon_status.media_attachments[0].clone();

            let media_embeds = match first_media_attachment.r#type {
                // Handle image attachments.
                megalodon::entities::attachment::AttachmentType::Image => {
                    self.generate_image_embed(&parsed_status.mastodon_status.media_attachments)
                        .await?
                }

                // Handle video attachments.
                megalodon::entities::attachment::AttachmentType::Video => {
                    self.generate_video_embed(&first_media_attachment).await?
                }

                // All other media types are unsupported.
                _ => {
                    tracing::warn!(
                        "Unsupported media type '{}' for post '{}'",
                        first_media_attachment.r#type,
                        parsed_status.mastodon_status.id
                    );

                    None
                }
            };

            self.post_item.embed = media_embeds;
        }

        // Create a vector to hold the richtext facets for the post item that will be
        // generated from any tags or links found in the post.
        let mut richtext_facets = Vec::new();

        // Check if the post has any tags/hashtags.
        if parsed_status.found_tags.len() > 0 {
            let rich_text_tags = self.generate_rich_text_tags(&parsed_status)?;

            richtext_facets.extend(rich_text_tags);
        }

        // Check if the post has any links.
        if parsed_status.found_links.len() > 0 {
            tracing::info!(
                "Found '{}' links in post '{}'",
                parsed_status.found_links.len(),
                self.mastodon_status.id
            );

            let rich_text_links = self.generate_rich_text_links(&parsed_status).await?;

            richtext_facets.extend(rich_text_links);
        }

        // Set the richtext facets for the post item if any were generated.
        self.post_item.facets = match richtext_facets.len() > 0 {
            true => Some(richtext_facets),
            false => None
        };

        Ok(())
    }
}
