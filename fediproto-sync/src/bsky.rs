use atprotolib_rs::{
    api_calls::{ApiAuthBearerToken, ApiAuthConfig, ApiAuthConfigData},
    types::{
        app_bsky::{
            self,
            embed::{ExternalEmbed, ImageEmbed},
            feed::{PostEmbedExternal, PostEmbedImage, PostEmbedVideo, PostEmbeds},
            richtext::{
                ByteSlice,
                RichTextFacet,
                RichTextFacetFeature,
                RichTextFacetLink,
                RichTextFacetTag
            }
        },
        com_atproto::{self, server::CreateSessionRequest}
    }
};
use chrono::Utc;
use diesel::*;
use rand::distributions::DistString;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

use crate::{mastodon, models, schema, FediProtoSyncEnvVars};

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

        // Create a vector to hold the richtext facets for the post item that will be
        // generated from any tags or links found in the post.
        let mut richtext_facets = Vec::new();

        // Check if the post has any media attachments and upload them to Bluesky.
        if parsed_status.mastodon_status.media_attachments.len() > 0 {
            tracing::info!(
                "Found '{}' media attachments in post '{}'",
                parsed_status.mastodon_status.media_attachments.len(),
                parsed_status.mastodon_status.id
            );

            let mut image_attachments = Vec::new();

            // Iterate over each media attachment in the Mastodon status.
            for media_attachment in parsed_status.mastodon_status.media_attachments.clone() {
                match media_attachment.r#type {
                    // Handle image attachments.
                    megalodon::entities::attachment::AttachmentType::Image => {
                        // Download the media attachment from the Mastodon server.
                        let media_attachment_client =
                            crate::core::create_http_client(&self.config)?;
                        let media_attachment_response = media_attachment_client
                            .get(&media_attachment.url)
                            .send()
                            .await?;
                        let media_attachment_bytes = media_attachment_response.bytes().await?;

                        let blob_upload_client = crate::core::create_http_client(&self.config)?;
                        // Upload the media attachment to Bluesky.
                        let blob_upload_response = com_atproto::repo::upload_blob(
                            &self.bsky_auth.host_name,
                            blob_upload_client,
                            &self.bsky_auth.auth_config,
                            media_attachment_bytes.to_vec(),
                            Some("image/jpeg")
                        )
                        .await?;

                        // Create an image embed and add it to the list of image attachments.
                        image_attachments.push(ImageEmbed {
                            image: blob_upload_response.blob,
                            alt: media_attachment
                                .description
                                .unwrap_or_else(|| "".to_string()),
                            aspect_ratio: None
                        });

                        tracing::info!(
                            "Uploaded media attachment '{}' to Bluesky",
                            media_attachment.url
                        );
                    }

                    // Handle video attachments.
                    megalodon::entities::attachment::AttachmentType::Video => {
                        let video_embed = self.generate_video_embed(&media_attachment).await?;

                        self.post_item.embed = video_embed;
                    }

                    _ => {
                        tracing::warn!(
                            "Unsupported media attachment type '{}' in post '{}'",
                            media_attachment.r#type,
                            parsed_status.mastodon_status.id
                        );
                    }
                }
            }

            if image_attachments.len() > 0 {
                // Add the image attachments to the post item as a post embed.
                self.post_item.embed = Some(PostEmbeds::Images(PostEmbedImage {
                    images: image_attachments
                }));
            }
        }

        // Check if the post has any tags/hashtags.
        if parsed_status.found_tags.len() > 0 {
            // Iterate over each tag found in the post and create a richtext facet for it.
            for tag in parsed_status.found_tags {
                // Find the start and end index of the tag in the post content to
                // generate a ByteSlice for the richtext facet.
                let tag_start_index = parsed_status.stripped_html.find(tag.as_str()).unwrap();
                let tag_end_index = tag_start_index + tag.len();

                // Create a richtext facet for the tag and add it to the list of richtext
                // facets.
                let richtext_facet_tag = RichTextFacet {
                    index: ByteSlice {
                        byte_start: tag_start_index as i64,
                        byte_end: tag_end_index as i64
                    },
                    features: vec![RichTextFacetFeature::Tag(RichTextFacetTag {
                        tag: tag.trim_start_matches("#").to_string()
                    })]
                };

                richtext_facets.push(richtext_facet_tag);
            }
        }

        // Check if the post has any links.
        if parsed_status.found_links.len() > 0 {
            tracing::info!(
                "Found '{}' links in post '{}'",
                parsed_status.found_links.len(),
                self.mastodon_status.id
            );

            // Get the first link found in the post.
            let first_link = parsed_status.found_links[0].clone();

            // Get metadata for the link.
            let link_metadata = self.get_link_metadata(&first_link).await?;

            // Check if the post has an embed and add an external embed for the link if
            // it doesn't.
            if self.post_item.embed.is_none() {
                tracing::info!(
                    "Post has no embeds, adding external embed for link '{}'",
                    first_link
                );

                // Get the thumbnail for the link if it has one and upload it to BlueSky.
                let link_thumbnail_url = link_metadata["image"].as_str().unwrap_or_else(|| "");
                let link_thumbnail_bytes = match link_thumbnail_url == "" {
                    true => vec![],
                    false => self.get_link_thumbnail(link_thumbnail_url).await?
                };

                let blob_item = match link_thumbnail_bytes.len() > 0 {
                    true => {
                        let blob_upload_client = crate::core::create_http_client(&self.config)?;
                        Some(
                            com_atproto::repo::upload_blob(
                                &self.bsky_auth.host_name,
                                blob_upload_client,
                                &self.bsky_auth.auth_config,
                                link_thumbnail_bytes,
                                Some("image/jpeg")
                            )
                            .await?
                            .blob
                        )
                    }

                    _ => None
                };

                // Create an external embed for the link and add it to the post item.
                self.post_item.embed = Some(PostEmbeds::External(PostEmbedExternal {
                    external: ExternalEmbed {
                        uri: link_metadata["url"].as_str().unwrap().to_string(),
                        title: link_metadata["title"].as_str().unwrap().to_string(),
                        description: link_metadata["description"].as_str().unwrap().to_string(),
                        thumb: blob_item
                    }
                }));
            }

            // Find the start and end index of the first link in the post content to
            // generate a ByteSlice for the richtext facet and add it to the list of
            // richtext facets for the post item.
            let link_start_index = parsed_status
                .stripped_html
                .find(first_link.as_str())
                .unwrap();
            let link_end_index = link_start_index + first_link.len();

            let richtext_facet_link = RichTextFacet {
                index: ByteSlice {
                    byte_start: link_start_index as i64,
                    byte_end: link_end_index as i64
                },
                features: vec![RichTextFacetFeature::Link(RichTextFacetLink {
                    uri: first_link
                })]
            };

            richtext_facets.push(richtext_facet_link);
        }

        // Set the richtext facets for the post item if any were generated.
        self.post_item.facets = match richtext_facets.len() > 0 {
            true => Some(richtext_facets),
            false => None
        };

        Ok(())
    }

    /// Generates a video embed for a BlueSky post from a media attachment from
    /// a Mastodon status.
    ///
    /// ## Arguments
    ///
    /// * `config` - The environment variables for the FediProtoSync
    ///   application.
    /// * `bsky_auth` - The Bluesky authentication information.
    /// * `host_name` - The hostname of the Bluesky/ATProto PDS.
    /// * `mastodon_status` - The Mastodon status to generate the video embed
    ///   from.
    /// * `media_attachment` - The media attachment to generate the video embed
    ///   from.
    async fn generate_video_embed(
        &mut self,
        media_attachment: &megalodon::entities::attachment::Attachment
    ) -> Result<Option<app_bsky::feed::PostEmbeds>, Box<dyn std::error::Error>> {
        #[allow(unused_assignments)]
        let mut post_embed: Option<app_bsky::feed::PostEmbeds> = None;

        let media_attachment_meta = media_attachment.meta.clone().unwrap();
        let video_duration = &media_attachment_meta.original.unwrap().duration.unwrap();

        match self.config.bluesky_video_always_fallback || *video_duration >= 60 as f64 {
            true => {
                let video_link_thumbnail_bytes = self
                    .get_link_thumbnail(media_attachment.preview_url.clone().unwrap().as_str())
                    .await?;

                let blob_item = match video_link_thumbnail_bytes.len() > 0 {
                    true => {
                        let blob_upload_client = crate::core::create_http_client(&self.config)?;
                        Some(
                            com_atproto::repo::upload_blob(
                                &self.bsky_auth.host_name,
                                blob_upload_client,
                                &self.bsky_auth.auth_config,
                                video_link_thumbnail_bytes,
                                Some("image/jpeg")
                            )
                            .await?
                            .blob
                        )
                    }

                    _ => None
                };

                post_embed = Some(PostEmbeds::External(PostEmbedExternal {
                    external: ExternalEmbed {
                        uri: self.mastodon_status.url.clone().unwrap(),
                        title: "View video on Mastodon".to_string(),
                        description: format!(
                            "Check out this video posted by @{}!",
                            self.mastodon_status.account.username.clone()
                        ),
                        thumb: blob_item
                    }
                }));
            }

            false => {
                // Download the media attachment from the Mastodon server.
                tracing::info!(
                    "Downloading video attachment '{}' from Mastodon",
                    media_attachment.url
                );

                let media_attachment_client = crate::core::create_http_client(&self.config)?;
                let mut media_attachment_response = media_attachment_client
                    .get(&media_attachment.url)
                    .send()
                    .await?;

                let temp_path = std::env::temp_dir().join(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 14));
                let mut temp_file = tokio::fs::File::create(&temp_path).await?;

                while let Some(chunk) = media_attachment_response.chunk().await? {
                    temp_file.write_all(&chunk).await?;
                }

                temp_file.flush().await?;

                let service_endpoint = self.get_pds_service_endpoint()?;
                let service_endpoint = service_endpoint.replace("https://", "");

                let service_auth_client = crate::core::create_http_client(&self.config)?;
                let service_auth_token = com_atproto::server::get_service_auth(
                    &service_endpoint,
                    service_auth_client,
                    &self.bsky_auth.auth_config,
                    format!("did:web:{}", &service_endpoint).as_str(),
                    (Utc::now() + chrono::Duration::minutes(30)).timestamp(),
                    Some("com.atproto.repo.uploadBlob")
                )
                .await?;

                let upload_auth_config = ApiAuthConfig {
                    data: ApiAuthConfigData::BearerToken(ApiAuthBearerToken {
                        token: service_auth_token.token.clone()
                    })
                };

                let random_video_name = format!(
                    "{}.mp4",
                    rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 14)
                );

                // Upload the video to BlueSky.
                tracing::info!(
                    "Uploading video attachment '{}' to Bluesky as '{}'",
                    media_attachment.url,
                    random_video_name
                );

                temp_file = tokio::fs::File::open(&temp_path).await?;
                let mut media_attachment_buffer = Vec::new();

                temp_file.read_to_end(&mut media_attachment_buffer).await?;

                let upload_video_client = crate::core::create_http_client(&self.config)?;
                let upload_video_job_response = app_bsky::video::upload_video(
                    "video.bsky.app",
                    upload_video_client,
                    &upload_auth_config,
                    media_attachment_buffer,
                    &self.bsky_auth.session.did,
                    &random_video_name
                )
                .await?;

                temp_file.flush().await?;
                tokio::fs::remove_file(&temp_path).await?;

                tracing::info!(
                    "Waiting for video upload job '{}' to complete",
                    upload_video_job_response.job_id
                );

                let no_auth_config = ApiAuthConfig {
                    data: ApiAuthConfigData::None
                };

                let mut job_status = upload_video_job_response.clone();

                while job_status.state != "JOB_STATE_FAILED" {
                    let job_client = crate::core::create_http_client(&self.config)?;
                    job_status = app_bsky::video::get_job_status(
                        "video.bsky.app",
                        job_client,
                        &no_auth_config,
                        &upload_video_job_response.job_id.as_str()
                    )
                    .await?
                    .job_status;

                    if job_status.state == "JOB_STATE_COMPLETED" {
                        break;
                    }

                    tracing::info!("Video upload progress: {}%", job_status.progress);

                    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                }

                match job_status.state.as_str() {
                    "JOB_STATE_FAILED" => {
                        tracing::error!(
                            "Failed to upload video attachment '{}'. Error message: '{}'",
                            media_attachment.url,
                            job_status.error.unwrap_or_else(|| "N/A".to_string())
                        );

                        return Err(Box::new(crate::error::Error::new(
                            "The BlueSky upload job failed.",
                            crate::error::ErrorKind::VideoUploadError
                        )));
                    }

                    _ => {
                        tracing::info!(
                            "Uploaded video attachment '{}' to Bluesky",
                            media_attachment.url
                        );

                        post_embed = Some(PostEmbeds::Video(PostEmbedVideo {
                            aspect_ratio: None,
                            video: job_status.blob.unwrap()
                        }))
                    }
                }
            }
        };

        Ok(post_embed)
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
