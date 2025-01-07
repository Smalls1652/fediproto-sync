mod media;
mod rich_text;
mod utils;

use std::str::FromStr;

use anyhow::Result;
use atrium_api::{
    agent::{store::MemorySessionStore, AtpAgent},
    app,
    com,
    types::{
        string::{Cid, Datetime, Nsid},
        TryIntoUnknown
    }
};
use atrium_xrpc_client::reqwest::ReqwestClient;
use diesel::r2d2::{ConnectionManager, Pool};
use fediproto_sync_db::{
    models::{NewMastodonPost, NewSyncedPostBlueSkyData},
    AnyConnection
};
use ipld_core::ipld::Ipld;

#[allow(unused_imports)]
pub use self::{
    media::{BlueSkyPostSyncMedia, MAX_IMAGE_SIZE, MAX_VIDEO_DURATION, MAX_VIDEO_SIZE},
    rich_text::BlueSkyPostSyncRichText,
    utils::BlueSkyPostSyncUtils
};
use crate::FediProtoSyncConfig;

/// Struct to hold the data and logic for syncing a Mastodon post to BlueSky.
pub struct BlueSkyPostSync<'a> {
    /// The environment variables for the FediProto Sync application.
    pub config: FediProtoSyncConfig,

    /// The authentication session for BlueSky.
    pub atp_agent: &'a AtpAgent<MemorySessionStore, ReqwestClient>,

    /// The DID of the BlueSky session.
    pub did: atrium_api::types::string::Did,

    /// The PDS service endpoint for the BlueSky session.
    pub pds_service_endpoint: String,

    /// The database connection for the FediProto Sync application.
    pub db_connection_pool: Pool<ConnectionManager<AnyConnection>>,

    /// The Mastodon account that posted the status.
    pub mastodon_account: megalodon::entities::account::Account,

    /// The Mastodon status.
    pub mastodon_status: megalodon::entities::Status,

    /// The post generated from the Mastodon status to sync to BlueSky.
    pub post_item: atrium_api::app::bsky::feed::post::RecordData
}

impl BlueSkyPostSync<'_> {
    /// Sync a Mastodon post to Bluesky.
    pub async fn sync_post(&mut self) -> Result<()> {
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
            ) {
                true => (),
                false => {
                    return Err(anyhow::anyhow!(
                        "Previous post '{}' not found in database for post '{}'",
                        in_reply_to_id,
                        self.mastodon_status.id
                    ));
                }
            }

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
            self.post_item.reply = Some(
                app::bsky::feed::post::ReplyRefData {
                    root: com::atproto::repo::strong_ref::MainData {
                        cid: Cid::from_str(&previous_synced_post_root.bsky_post_cid)?,
                        uri: previous_synced_post_root.bsky_post_uri.clone()
                    }
                    .into(),
                    parent: com::atproto::repo::strong_ref::MainData {
                        cid: Cid::from_str(&previous_synced_post.bsky_post_cid)?,
                        uri: previous_synced_post.bsky_post_uri.clone()
                    }
                    .into()
                }
                .into()
            );
        }

        let collection = Nsid::new("app.bsky.feed.post".to_string()).map_err(|_| {
            anyhow::anyhow!("Error creating NSID for collection 'app.bsky.feed.post'")
        })?;

        // -- Send the post item to BlueSky through the 'com.atproto.repo.applyWrites'
        // API. --
        let apply_writes_result = self
            .atp_agent
            .api
            .com
            .atproto
            .repo
            .apply_writes(
                com::atproto::repo::apply_writes::InputData {
                    repo: atrium_api::types::string::AtIdentifier::Did(self.did.clone()),
                    writes: vec![com::atproto::repo::apply_writes::InputWritesItem::Create(
                        Box::new(
                            com::atproto::repo::apply_writes::Create {
                                data: com::atproto::repo::apply_writes::CreateData {
                                    collection: collection,
                                    rkey: None,
                                    value: self.post_item.clone().try_into_unknown()?
                                },
                                extra_data: Ipld::Null
                            }
                            .into()
                        )
                    )],
                    swap_commit: None,
                    validate: Some(true)
                }
                .into()
            )
            .await;

        // -- Handle the response from the 'com.atproto.repo.applyWrites' API. --
        match apply_writes_result {
            Ok(result) => {
                // If no HTTP errors occurred, get the results from the response.
                // We need the CID and URI of the post that was created from it.
                let post_result = result.results.clone().unwrap();
                let post_result = match post_result.first().unwrap() {
                    com::atproto::repo::apply_writes::OutputResultsItem::CreateResult(
                        create_result
                    ) => create_result,

                    _ => panic!("Unexpected response from Bluesky")
                };

                let new_mastodon_post = NewMastodonPost::new(
                    &self.mastodon_status,
                    Some(post_result.cid.clone().as_ref().to_string()),
                    previous_post_id
                );

                let new_synced_post = NewSyncedPostBlueSkyData::new(
                    &self.mastodon_status.id,
                    &post_result.cid.clone().as_ref().to_string(),
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
                Err(error.into())
            }
        }
    }

    /// Generate a Bluesky post item from a Mastodon status.
    pub async fn generate_post_item(&mut self) -> Result<()> {
        // -- Parse the Mastodon status for post content and metadata. --
        let mut parsed_status =
            crate::mastodon::ParsedMastodonPost::from_mastodon_status(&self.mastodon_status)?;

        // -- Create a Bluesky post item from the parsed Mastodon status. --

        // Truncate the post content to fit within the 300 character limit of Bluesky.
        parsed_status.truncate_post_content()?;

        self.post_item = atrium_api::app::bsky::feed::post::RecordData {
            created_at: Datetime::new(parsed_status.mastodon_status.created_at.fixed_offset()),
            text: parsed_status.stripped_html.clone(),
            langs: None,
            embed: None,
            facets: None,
            entities: None,
            labels: None,
            reply: None,
            tags: None
        };

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
