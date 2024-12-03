use atprotolib_rs::{
    api_calls::{ApiAuthBearerToken, ApiAuthConfig, ApiAuthConfigData},
    types::{
        app_bsky::{
            self,
            embed::{ExternalEmbed, ImageEmbed},
            feed::{PostEmbedExternal, PostEmbedImage, PostEmbeds},
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
use diesel::*;

use crate::{mastodon, models, schema, FediProtoSyncEnvVars};

/// Holds the authentication information for a Bluesky session.
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
    config: &FediProtoSyncEnvVars
) -> Result<BlueSkyAuthentication, Box<dyn std::error::Error>> {
    let initial_auth_config = ApiAuthConfig {
        data: ApiAuthConfigData::None
    };

    let bsky_create_session = atprotolib_rs::types::com_atproto::server::create_session(
        &config.bluesky_pds_server,
        &initial_auth_config,
        CreateSessionRequest {
            identifier: config.bluesky_handle.clone(),
            password: config.bluesky_app_password.clone(),
            auth_factor_token: None
        }
    )
    .await?;

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

/// Sync a Mastodon post to Bluesky.
///
/// ## Arguments
///
/// * `bsky_auth` - The Bluesky authentication information.
/// * `db_connection` - The SQLite database connection.
/// * `mastodon_account` - The Mastodon account that posted the status.
/// * `mastodon_status` - The Mastodon status to sync.
pub async fn sync_post(
    bsky_auth: &BlueSkyAuthentication,
    db_connection: &mut diesel::SqliteConnection,
    mastodon_account: &megalodon::entities::account::Account,
    mastodon_status: &megalodon::entities::Status
) -> Result<(), Box<dyn std::error::Error>> {
    let mastodon_status = mastodon_status.clone();

    // -- Generate a BlueSky post item from the Mastodon status. --
    let mut post_item = generate_post_item(
        &bsky_auth.host_name,
        &bsky_auth.auth_config,
        &mastodon_status
    )
    .await?;

    // -- Check if the post is a reply to another post in a thread. --
    let mut previous_post_id = None;

    // If the post has a "in_reply_to_id" field and the "in_reply_to_account_id"
    // field is the same as the account ID of the account that posted the status,
    // then it is potentially a reply to another post in a thread.
    if mastodon_status.in_reply_to_id.is_some()
        && mastodon_status
            .clone()
            .in_reply_to_account_id
            .unwrap_or_else(|| "".to_string())
            == mastodon_account.id.clone()
    {
        let in_reply_to_id = mastodon_status.in_reply_to_id.clone().unwrap();

        // Resolve the previous post in the thread and resolve it's synced post data.
        let previous_mastodon_post = resolve_previous_post(db_connection, &in_reply_to_id).await?;
        let previous_synced_post = get_synced_post(db_connection, &in_reply_to_id).await?;

        // If the previous post has a root post, resolve it's synced post data.
        let previous_synced_post_root = match previous_mastodon_post.root_mastodon_post_id {
            Some(root_mastodon_post_id) => {
                // Set the previous post ID to the root post ID retrieved.
                previous_post_id = Some(root_mastodon_post_id.clone());

                get_synced_post(db_connection, &root_mastodon_post_id).await?
            }
            None => {
                // Set the previous post ID to the previous post ID.
                previous_post_id = Some(in_reply_to_id.clone());

                previous_synced_post.clone()
            }
        };

        // Set the reply reference for the post item.
        post_item.reply_ref = Some(app_bsky::feed::PostReplyRef {
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
        repo: bsky_auth.session.did.clone(),
        validate: true,
        writes: vec![com_atproto::repo::ApplyWritesRequestWrites::Create(
            com_atproto::repo::Create::new(
                "app.bsky.feed.post",
                com_atproto::repo::ApplyWritesValue::Post(post_item)
            )
        )],
        swap_commit: None
    };

    let apply_write_result = com_atproto::repo::apply_writes(
        &bsky_auth.host_name,
        &bsky_auth.auth_config,
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
                    &mastodon_status,
                    Some(post_result.cid.clone()),
                    previous_post_id
                ))
                .execute(db_connection)?;

            // Insert the synced BlueSky post into the database for future tracking.
            diesel::insert_into(schema::synced_posts::table)
                .values(models::NewSyncedPost::new(
                    &mastodon_status.id,
                    &post_result.cid,
                    &post_result.uri
                ))
                .execute(db_connection)?;

            tracing::info!("Synced post '{}' to BlueSky.", mastodon_status.id);

            Ok(())
        }

        Err(error) => {
            // If an error occurred, log the error and return it.
            tracing::error!("Error syncing post '{}': {}", mastodon_status.id, error);
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
/// * `mastodon_status` - The Mastodon status to generate the post item from.
pub async fn generate_post_item(
    host_name: &str,
    auth_config: &ApiAuthConfig,
    mastodon_status: &megalodon::entities::Status
) -> Result<app_bsky::feed::Post, Box<dyn std::error::Error>> {
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
    mastodon_status_send.send(mastodon_status.clone()).await?;

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
    let mut post_item = app_bsky::feed::Post::new(
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

        // Create a HTTP client for downloading the media attachments.
        let media_attachment_client = reqwest::Client::new();

        // Iterate over each media attachment in the Mastodon status.
        for media_attachment in parsed_status.mastodon_status.media_attachments {
            // Skip any media attachments that aren't images.
            if media_attachment.r#type != megalodon::entities::attachment::AttachmentType::Image {
                tracing::warn!(
                    "Skipping non-image media attachment '{}'",
                    media_attachment.url
                );
                continue;
            }

            // Download the media attachment from the Mastodon server.
            let media_attachment_response = media_attachment_client
                .get(&media_attachment.url)
                .send()
                .await?;
            let media_attachment_bytes = media_attachment_response.bytes().await?;

            // Upload the media attachment to Bluesky.
            let blob_upload_response = com_atproto::repo::upload_blob(
                host_name,
                auth_config,
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

        // Add the image attachments to the post item as a post embed.
        post_item.embed = Some(PostEmbeds::Images(PostEmbedImage {
            images: image_attachments
        }));
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
            mastodon_status.id
        );

        // Get the first link found in the post.
        let first_link = parsed_status.found_links[0].clone();

        // Get metadata for the link.
        let link_metadata = get_link_metadata(&first_link).await?;

        // Check if the post has an embed and add an external embed for the link if
        // it doesn't.
        if post_item.embed.is_none() {
            tracing::info!(
                "Post has no embeds, adding external embed for link '{}'",
                first_link
            );

            // Get the thumbnail for the link if it has one and upload it to BlueSky.
            let link_thumbnail_url = link_metadata["image"].as_str().unwrap_or_else(|| "");
            let link_thumbnail_bytes = match link_thumbnail_url == "" {
                true => vec![],
                false => get_link_thumbnail(link_thumbnail_url).await?
            };

            let blob_item = match link_thumbnail_bytes.len() > 0 {
                true => Some(
                    com_atproto::repo::upload_blob(
                        host_name,
                        auth_config,
                        link_thumbnail_bytes,
                        Some("image/jpeg")
                    )
                    .await?
                    .blob
                ),

                _ => None
            };

            // Create an external embed for the link and add it to the post item.
            post_item.embed = Some(PostEmbeds::External(PostEmbedExternal {
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
    post_item.facets = match richtext_facets.len() > 0 {
        true => Some(richtext_facets),
        false => None
    };

    Ok(post_item)
}

/// Resolve the previous post in a thread on Mastodon based off what has been
/// previously synced.
///
/// ## Arguments
///
/// * `db_connection` - The SQLite database connection.
/// * `previous_post_id` - The Mastodon post ID of the previous post in the
///   thread.
pub async fn resolve_previous_post(
    db_connection: &mut diesel::SqliteConnection,
    previous_post_id: &str
) -> Result<models::MastodonPost, Box<dyn std::error::Error>> {
    let previous_post = schema::mastodon_posts::table
        .filter(schema::mastodon_posts::post_id.eq(previous_post_id))
        .first::<models::MastodonPost>(db_connection)?;

    Ok(previous_post)
}

/// Get a synced post from the database.
///
/// ## Arguments
///
/// * `db_connection` - The SQLite database connection.
/// * `mastodon_post_id` - The Mastodon post ID of the post to get.
pub async fn get_synced_post(
    db_connection: &mut diesel::SqliteConnection,
    mastodon_post_id: &str
) -> Result<models::SyncedPost, Box<dyn std::error::Error>> {
    let synced_post = schema::synced_posts::table
        .filter(schema::synced_posts::mastodon_post_id.eq(mastodon_post_id))
        .first::<models::SyncedPost>(db_connection)?;

    Ok(synced_post)
}

/// Get link metadata from the CardyB API.
///
/// ## Arguments
///
/// * `url` - The URL to get metadata for.
pub async fn get_link_metadata(url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    tracing::info!("Getting link metadata for '{}'.", url);
    let client = reqwest::Client::new();

    let query_params = vec![("url", url)];

    let link_info_response = client
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
pub async fn get_link_thumbnail(image_url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    tracing::info!("Getting link thumbnail for '{}'.", image_url);
    let client = reqwest::Client::new();

    let link_thumbnail_response = client.get(image_url).send().await?;

    let link_thumbnail_bytes = link_thumbnail_response.bytes().await?;

    Ok(link_thumbnail_bytes.to_vec())
}
