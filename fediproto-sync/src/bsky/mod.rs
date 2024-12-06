mod media;
mod rich_text;

use atprotolib_rs::{
    api_calls::{ApiAuthBearerToken, ApiAuthConfig, ApiAuthConfigData},
    types::{
        app_bsky,
        com_atproto::{self, server::CreateSessionRequest}
    }
};
use diesel::*;

use crate::{
    bsky::{media::BlueSkyPostSyncMedia, rich_text::BlueSkyPostSyncRichText},
    mastodon,
    models,
    schema,
    FediProtoSyncEnvVars
};

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
    /// * `host_name` - The hostname of the BlueSky/ATProto PDS.
    /// * `auth_config` - The API authentication configuration for the session.
    /// * `session` - The session information for the authenticated BlueSky
    ///   session.
    pub fn new(
        host_name: &str,
        auth_config: ApiAuthConfig,
        session: com_atproto::server::CreateSessionResponse
    ) -> Self {
        Self {
            host_name: host_name.to_string(),
            auth_config,
            session
        }
    }
}

/// Create a new Bluesky session token.
///
/// ## Arguments
///
/// * `config` - The environment variables for the FediProtoSync application.
pub async fn create_session_token(
    config: &FediProtoSyncEnvVars,
    client: reqwest::Client
) -> Result<BlueSkyAuthentication, crate::error::Error> {
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
        crate::error::Error::with_source(
            "Failed to create Bluesky session.",
            crate::error::ErrorKind::AuthenticationError,
            e
        )
    })?;

    let bsky_auth_config = ApiAuthConfig {
        data: ApiAuthConfigData::BearerToken(ApiAuthBearerToken {
            token: bsky_create_session.access_jwt.clone()
        })
    };

    Ok(BlueSkyAuthentication::new(
        config.bluesky_pds_server.clone().as_str(),
        bsky_auth_config,
        bsky_create_session
    ))
}

pub async fn refresh_session_token(
    bsky_auth: &BlueSkyAuthentication,
    client: reqwest::Client
) -> Result<BlueSkyAuthentication, crate::error::Error> {
    let refresh_auth_config = ApiAuthConfig {
        data: ApiAuthConfigData::BearerToken(ApiAuthBearerToken {
            token: bsky_auth.session.refresh_jwt.clone()
        })
    };

    let bsky_refresh_session =
        com_atproto::server::refresh_session(&bsky_auth.host_name, client, &refresh_auth_config)
            .await
            .map_err(|e| {
                crate::error::Error::with_source(
                    "Failed to refresh Bluesky session.",
                    crate::error::ErrorKind::AuthenticationError,
                    e
                )
            })?;

    let bsky_auth_config = ApiAuthConfig {
        data: ApiAuthConfigData::BearerToken(ApiAuthBearerToken {
            token: bsky_refresh_session.access_jwt.clone()
        })
    };

    Ok(BlueSkyAuthentication::new(
        &bsky_auth.host_name,
        bsky_auth_config,
        bsky_refresh_session
    ))
}

pub struct BlueSkyPostSync<'a> {
    pub config: FediProtoSyncEnvVars,
    pub bsky_auth: BlueSkyAuthentication,
    pub db_connection: &'a mut diesel::SqliteConnection,
    pub mastodon_account: megalodon::entities::account::Account,
    pub mastodon_status: megalodon::entities::Status,
    pub post_item: app_bsky::feed::Post
}

impl BlueSkyPostSync<'_> {
    /// Sync a Mastodon post to Bluesky.
    ///
    /// ## Arguments
    ///
    /// * `bsky_auth` - The Bluesky authentication information.
    /// * `db_connection` - The SQLite database connection.
    /// * `mastodon_account` - The Mastodon account that posted the status.
    /// * `mastodon_status` - The Mastodon status to sync.
    pub async fn sync_post(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

            // Resolve the previous post in the thread and resolve it's synced post data.
            let previous_mastodon_post = self.resolve_previous_post(&in_reply_to_id).await?;
            let previous_synced_post = self.get_synced_post(&in_reply_to_id).await?;

            // If the previous post has a root post, resolve it's synced post data.
            let previous_synced_post_root = match previous_mastodon_post.root_mastodon_post_id {
                Some(root_mastodon_post_id) => {
                    // Set the previous post ID to the root post ID retrieved.
                    previous_post_id = Some(root_mastodon_post_id.clone());

                    self.get_synced_post(&root_mastodon_post_id).await?
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

                // Insert the synced Mastodon post into the database for future tracking.
                diesel::insert_into(schema::mastodon_posts::table)
                    .values(models::NewMastodonPost::new(
                        &self.mastodon_status,
                        Some(post_result.cid.clone()),
                        previous_post_id
                    ))
                    .execute(self.db_connection)?;

                // Insert the synced BlueSky post into the database for future tracking.
                diesel::insert_into(schema::synced_posts::table)
                    .values(models::NewSyncedPost::new(
                        &self.mastodon_status.id,
                        &post_result.cid,
                        &post_result.uri
                    ))
                    .execute(self.db_connection)?;

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
    ///
    /// ## Arguments
    ///
    /// * `host_name` - The hostname of the Bluesky/ATProto PDS.
    /// * `auth_config` - The API authentication configuration for the session.
    /// * `mastodon_status` - The Mastodon status to generate the post item
    ///   from.
    pub async fn generate_post_item(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // -- Parse the Mastodon status for post content and metadata. --

        //
        // Note:
        //
        // The following code block is a workaround for the HTML parsing library we're
        // using. The `dom_query` crate has an underlying crate, `tendril`, that isn't
        // thread safe. In order to use it, we have to spawn a blocking thread to parse
        // it. In this context, it's not a big deal. We're only parsing one post at a
        // time and we don't intend to parse multiple at once in the future (A queued up
        // post may require another post to be processed before it can).

        // Create channels for sending and receiving the Mastodon status and parsed post
        // to/from the blocking thread.
        let (mastodon_status_send, mut mastodon_status_recv) = tokio::sync::mpsc::channel(1);
        let (parsed_send, mut parsed_recv) = tokio::sync::mpsc::channel(1);

        // Send the Mastodon status to the blocking thread for parsing.
        mastodon_status_send
            .send(self.mastodon_status.clone())
            .await?;

        // Spawn a blocking thread to parse the Mastodon status.
        tokio::task::spawn_blocking(move || {
            // Receive the Mastodon status from the non-blocking thread and close the
            // channel.
            let recieved_status = mastodon_status_recv.blocking_recv().unwrap();
            mastodon_status_recv.close();

            let parsed_status =
                mastodon::ParsedMastodonPost::from_mastodon_status(&recieved_status).unwrap();

            parsed_send.blocking_send(parsed_status).unwrap();
        });

        // Receive the parsed post from the blocking thread and close the channel.
        let mut parsed_status = parsed_recv.recv().await.unwrap();
        parsed_recv.close();

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

    /// Resolve the previous post in a thread on Mastodon based off what has
    /// been previously synced.
    ///
    /// ## Arguments
    ///
    /// * `db_connection` - The SQLite database connection.
    /// * `previous_post_id` - The Mastodon post ID of the previous post in the
    ///   thread.
    async fn resolve_previous_post(
        &mut self,
        previous_post_id: &str
    ) -> Result<models::MastodonPost, Box<dyn std::error::Error>> {
        let previous_post = schema::mastodon_posts::table
            .filter(schema::mastodon_posts::post_id.eq(previous_post_id))
            .first::<models::MastodonPost>(self.db_connection)?;

        Ok(previous_post)
    }

    /// Get a synced post from the database.
    ///
    /// ## Arguments
    ///
    /// * `db_connection` - The SQLite database connection.
    /// * `mastodon_post_id` - The Mastodon post ID of the post to get.
    async fn get_synced_post(
        &mut self,
        mastodon_post_id: &str
    ) -> Result<models::SyncedPost, Box<dyn std::error::Error>> {
        let synced_post = schema::synced_posts::table
            .filter(schema::synced_posts::mastodon_post_id.eq(mastodon_post_id))
            .first::<models::SyncedPost>(self.db_connection)?;

        Ok(synced_post)
    }

    /// Get link metadata from the CardyB API.
    ///
    /// ## Arguments
    ///
    /// * `url` - The URL to get metadata for.
    async fn get_link_metadata(
        &mut self,
        url: &str
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        tracing::info!("Getting link metadata for '{}'.", url);
        let query_params = vec![("url", url)];

        let link_info_client = crate::core::create_http_client(&self.config)?;

        let link_info_response = link_info_client
            .get("https://cardyb.bsky.app/v1/extract")
            .query(&query_params)
            .send()
            .await?;

        let link_info_json = link_info_response.json::<serde_json::Value>().await?;

        Ok(link_info_json)
    }

    /// Get a link thumbnail returned by the CardyB API.
    ///
    /// ## Arguments
    ///
    /// * `image_url` - The URL of the image to get.
    async fn get_link_thumbnail(
        &mut self,
        image_url: &str
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        tracing::info!("Getting link thumbnail for '{}'.", image_url);

        let link_thumbnail_client = crate::core::create_http_client(&self.config)?;

        let link_thumbnail_response = link_thumbnail_client.get(image_url).send().await?;

        let link_thumbnail_bytes = link_thumbnail_response.bytes().await?;

        Ok(link_thumbnail_bytes.to_vec())
    }

    /// Get the PDS service endpoint from the Bluesky session.
    fn get_pds_service_endpoint(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let service_endpoint = match self.bsky_auth.session.did_doc.clone() {
            Some(did_doc) => {
                let session_services = did_doc.service.clone();

                let pds_service = session_services.iter().find_map(|service| match service {
                    com_atproto::server::DidDocServices::AtprotoPersonalDataServer(pds_service) => {
                        Some(pds_service.clone())
                    }
                });

                match pds_service {
                    Some(pds_service) => pds_service.service_endpoint.clone(),
                    None => {
                        return Err(Box::new(crate::error::Error::new(
                            "No PDS service found in Bluesky session.",
                            crate::error::ErrorKind::AuthenticationError
                        )))
                    }
                }
            }

            None => self.bsky_auth.host_name.clone()
        };

        Ok(service_endpoint)
    }
}
