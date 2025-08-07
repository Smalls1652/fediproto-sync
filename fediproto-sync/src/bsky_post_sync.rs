use std::{num::NonZero, str::FromStr};

use anyhow::Result;
use atrium_api::{
    agent::atp_agent::{AtpAgent, store::MemorySessionStore},
    app::{
        self,
        bsky::{
            embed::{defs::AspectRatioData, images::ImageData},
            feed::post::RecordEmbedRefs,
        },
    },
    com,
    types::{
        CidLink, Object, TryIntoUnknown, Union,
        string::{Cid, Datetime, Did, Nsid},
    },
};
use atrium_xrpc_client::reqwest::ReqwestClient;
use diesel::r2d2::{ConnectionManager, Pool};
use fediproto_sync_db::{
    AnyConnection,
    models::{NewCachedFile, NewMastodonPost, NewSyncedPostBlueSkyData},
};
use fediproto_sync_lib::{error::FediProtoSyncError, utils::new_random_file_name};
use ipld_core::ipld::Ipld;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use crate::{
    FediProtoSyncConfig, core::create_http_client, img_utils::ImageAttachmentData,
    mastodon::ParsedMastodonPost,
};

/// The maximum duration for a BlueSky video in seconds.
///
/// (Currently `60` seconds)
pub const MAX_VIDEO_DURATION: f64 = 60.0;

/// The maximum size for a BlueSky image in bytes.
///
/// (Currently `976.56 KB`, but set to `950 KB` to account for overhead)
#[allow(dead_code)]
pub const MAX_IMAGE_SIZE: u64 = 950_000;

/// The maximum size for a BlueSky video in bytes.
///
/// (Currently `50 MB`)
pub const MAX_VIDEO_SIZE: u64 = 50_000_000;

/// Holds config data for syncing a single post.
pub struct BlueSkyPostSyncConfig {
    /// The environment variables for the FediProto Sync application.
    pub config: FediProtoSyncConfig,

    /// The DID of the BlueSky session.
    pub did: atrium_api::types::string::Did,

    /// The PDS service endpoint for the BlueSky session.
    pub pds_service_endpoint: String,

    /// The Mastodon account that posted the status.
    pub mastodon_account: megalodon::entities::account::Account,

    /// The database connection for the FediProto Sync application.
    pub db_connection_pool: Pool<ConnectionManager<AnyConnection>>,
}

/// Sync a Mastodon post to BlueSky.
///
/// ## Arguments
///
/// * `mastodon_status` - The Mastodon status.
/// * `atp_client` - The client/agent for interacting with the AT Protocol.
/// * `sync_config` - Config for the sync.
pub async fn sync_post(
    mastodon_status: &megalodon::entities::Status,
    atp_client: &AtpAgent<MemorySessionStore, ReqwestClient>,
    sync_config: &BlueSkyPostSyncConfig,
) -> Result<()> {
    let db_connection = &mut sync_config.db_connection_pool.get()?;

    let collection = Nsid::new("app.bsky.feed.post".to_string())
        .map_err(|_| anyhow::anyhow!("Error creating NSID for collection 'app.bsky.feed.post'"))?;

    let mut previous_post_id = None;
    let post_item = match mastodon_status.reblog.is_some() {
        true => process_boosted_post(mastodon_status, atp_client, sync_config).await?,
        false => {
            let (generated_post, previous_id) =
                process_post(mastodon_status, atp_client, sync_config).await?;

            previous_post_id = previous_id;

            generated_post
        }
    };

    let apply_writes_result = atp_client
        .api
        .com
        .atproto
        .repo
        .apply_writes(
            com::atproto::repo::apply_writes::InputData {
                repo: atrium_api::types::string::AtIdentifier::Did(sync_config.did.clone()),
                writes: vec![com::atproto::repo::apply_writes::InputWritesItem::Create(
                    Box::new(
                        com::atproto::repo::apply_writes::Create {
                            data: com::atproto::repo::apply_writes::CreateData {
                                collection: collection,
                                rkey: None,
                                value: post_item.try_into_unknown()?,
                            },
                            extra_data: Ipld::Null,
                        }
                        .into(),
                    ),
                )],
                swap_commit: None,
                validate: Some(true),
            }
            .into(),
        )
        .await?;

    // If no HTTP errors occurred, get the results from the response.
    // We need the CID and URI of the post that was created from it.
    let post_result = apply_writes_result.results.clone().unwrap();
    let post_result = match post_result.first().unwrap() {
        com::atproto::repo::apply_writes::OutputResultsItem::CreateResult(create_result) => {
            create_result
        }

        _ => panic!("Unexpected response from Bluesky"),
    };

    let new_mastodon_post = NewMastodonPost::new(
        &mastodon_status,
        Some(post_result.cid.clone().as_ref().to_string()),
        previous_post_id.clone(),
    );

    let new_synced_post = NewSyncedPostBlueSkyData::new(
        &mastodon_status.id,
        &post_result.cid.clone().as_ref().to_string(),
        &post_result.uri,
    );

    // Insert the synced Mastodon post into the database for future tracking.
    fediproto_sync_db::operations::insert_new_synced_mastodon_post(
        db_connection,
        &new_mastodon_post,
    )?;

    // Insert the synced BlueSky post into the database for future tracking.
    fediproto_sync_db::operations::insert_new_bluesky_data_for_synced_mastodon_post(
        db_connection,
        &new_synced_post,
    )?;

    tracing::info!("Synced post '{}' to BlueSky.", &mastodon_status.id);

    Ok(())
}

/// Process a regular Mastodon post and generate a BlueSky post for it.
///
/// ## Arguments
///
/// * `mastodon_status` - The Mastodon status.
/// * `atp_client` - The client/agent for interacting with the AT Protocol.
/// * `sync_config` - Config for the sync.
async fn process_post(
    mastodon_status: &megalodon::entities::Status,
    atp_client: &AtpAgent<MemorySessionStore, ReqwestClient>,
    sync_config: &BlueSkyPostSyncConfig,
) -> Result<(
    atrium_api::app::bsky::feed::post::RecordData,
    Option<String>,
)> {
    let mut post_item = generate_post_item(mastodon_status, atp_client, sync_config).await?;

    let mut previous_post_id = None;
    if let Some(reply_to_id) = &mastodon_status.in_reply_to_id {
        let reply_to_account_id = mastodon_status
            .in_reply_to_account_id
            .clone()
            .unwrap_or_else(|| "".to_string());

        if reply_to_account_id == sync_config.mastodon_account.id {
            if let Some(previous_post) = resolve_previous_post(&reply_to_id, sync_config).await? {
                post_item.reply = Some(previous_post.0.into());
                previous_post_id = Some(previous_post.1);
            }
        }
    }

    Ok((post_item, previous_post_id))
}

/// Process a boosted Mastodon post and generate a BlueSky post for it.
///
/// ## Arguments
///
/// * `mastodon_status` - The boosted Mastodon status.
/// * `atp_client` - The client/agent for interacting with the AT Protocol.
/// * `sync_config` - Config for the sync.
async fn process_boosted_post(
    mastodon_status: &megalodon::entities::Status,
    atp_client: &AtpAgent<MemorySessionStore, ReqwestClient>,
    sync_config: &BlueSkyPostSyncConfig,
) -> Result<atrium_api::app::bsky::feed::post::RecordData> {
    let mut post_item = atrium_api::app::bsky::feed::post::RecordData {
        created_at: Datetime::new(mastodon_status.created_at.fixed_offset()),
        text: "".to_string(),
        langs: None,
        embed: None,
        facets: None,
        entities: None,
        labels: None,
        reply: None,
        tags: None,
    };

    let reblogged_status = mastodon_status.reblog.clone().unwrap();

    let parsed_status =
        ParsedMastodonPost::from_mastodon_status(&reblogged_status)?.truncate_post_content()?;

    post_item.embed = generate_boost_link_embed(&parsed_status, atp_client, sync_config).await?;

    Ok(post_item)
}

/// Generate a BlueSky post from a Mastodon post.
///
/// ## Arguments
///
/// * `mastodon_status` - The Mastodon status to generate from.
/// * `atp_client` - The client/agent for interacting with the AT Protocol.
/// * `sync_config` - Config for the sync.
async fn generate_post_item(
    mastodon_status: &megalodon::entities::Status,
    atp_client: &AtpAgent<MemorySessionStore, ReqwestClient>,
    sync_config: &BlueSkyPostSyncConfig,
) -> Result<atrium_api::app::bsky::feed::post::RecordData> {
    // Parse the Mastodon post.
    let parsed_status =
        ParsedMastodonPost::from_mastodon_status(mastodon_status)?.truncate_post_content()?;

    // Create the BlueSky post item.
    let mut post_item = atrium_api::app::bsky::feed::post::RecordData {
        created_at: Datetime::new(parsed_status.mastodon_status.created_at.fixed_offset()),
        text: parsed_status.stripped_html.clone(),
        langs: None,
        embed: None,
        facets: None,
        entities: None,
        labels: None,
        reply: None,
        tags: None,
    };

    // Add media attachments.
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
                generate_image_embed(
                    &parsed_status.mastodon_status.media_attachments,
                    atp_client,
                    sync_config,
                )
                .await?
            }

            // Handle video attachments.
            megalodon::entities::attachment::AttachmentType::Video => {
                generate_video_embed(
                    &first_media_attachment,
                    mastodon_status,
                    atp_client,
                    sync_config,
                )
                .await?
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

        post_item.embed = media_embeds;
    }

    // Create richtext facets.
    let mut richtext_facets = Vec::<Object<app::bsky::richtext::facet::MainData>>::new();

    // Add hashtags to richtext facets.
    if parsed_status.found_tags.len() > 0 {
        richtext_facets.extend(generate_rich_text_tags(&parsed_status)?);
    }

    // Add links to richtext facets.
    if parsed_status.found_links.len() > 0 {
        richtext_facets.extend(generate_rich_text_links(&parsed_status)?);

        // Check if the post has an embed and add an external embed for the first link
        // if it doesn't.
        if post_item.embed.is_none() {
            // Get the first link found in the post.
            let first_link = parsed_status.found_links[0].clone();

            tracing::info!(
                "Post has no embeds, adding external embed for link '{}'",
                first_link
            );

            post_item.embed = generate_link_embed(&first_link, atp_client, sync_config).await?;
        }
    }

    post_item.facets = match richtext_facets.len() != 0 {
        true => Some(richtext_facets),
        false => None,
    };

    Ok(post_item)
}

/// Generate embed(s) for an image(s) to add to a BlueSky post.
///
/// ## Arguments
///
/// * `media_attachments` - A list of attachments from a Mastodon post.
/// * `atp_client` - The client/agent for interacting with the AT Protocol.
/// * `sync_config` - Config for the sync.
async fn generate_image_embed(
    media_attachments: &Vec<megalodon::entities::attachment::Attachment>,
    atp_client: &AtpAgent<MemorySessionStore, ReqwestClient>,
    sync_config: &BlueSkyPostSyncConfig,
) -> Result<Option<Union<RecordEmbedRefs>>> {
    let mut image_attachments = Vec::<Object<ImageData>>::new();

    for image_attachment in media_attachments {
        // Download the media attachment from the Mastodon server.
        let temp_file_path = download_file_to_temp(&image_attachment.url, &sync_config).await?;

        let media_attachment = ImageAttachmentData::new(
            tokio::fs::read(temp_file_path).await?.into(),
            &image_attachment.url,
        )?;

        tracing::info!(
            "Aspect ratio: {}:{}",
            media_attachment.aspect_ratio_width,
            media_attachment.aspect_ratio_height
        );

        let aspect_ratio_data = AspectRatioData {
            width: NonZero::<u64>::new(media_attachment.aspect_ratio_width as u64).unwrap(),
            height: NonZero::<u64>::new(media_attachment.aspect_ratio_height as u64).unwrap(),
        };

        tracing::info!("Uploading '{}' bytes", media_attachment.image_bytes.len());
        let blob_upload_response = atp_client
            .api
            .com
            .atproto
            .repo
            .upload_blob(media_attachment.image_bytes.into())
            .await?;

        // Create an image embed and add it to the list of image attachments.
        image_attachments.push(
            app::bsky::embed::images::ImageData {
                image: blob_upload_response.blob.clone(),
                alt: image_attachment
                    .description
                    .clone()
                    .unwrap_or_else(|| "".to_string()),
                aspect_ratio: Some(aspect_ratio_data.into()),
            }
            .into(),
        );

        tracing::info!(
            "Uploaded media attachment '{}' to Bluesky",
            image_attachment.url
        );
    }

    Ok(Some(Union::Refs(RecordEmbedRefs::AppBskyEmbedImagesMain(
        Box::new(
            app::bsky::embed::images::MainData {
                images: image_attachments,
            }
            .into(),
        ),
    ))))
}

/// Generate an embed for a video to add to a BlueSky post.
///
/// ## Arguments
///
/// * `media_attachment` - The video attachment to generate an embed for.
/// * `mastodon_status` - The Mastodon status that the attachment originates from.
/// * `atp_client` - The client/agent for interacting with the AT Protocol.
/// * `sync_config` - Config for the sync.
async fn generate_video_embed(
    media_attachment: &megalodon::entities::attachment::Attachment,
    mastodon_status: &megalodon::entities::Status,
    atp_client: &AtpAgent<MemorySessionStore, ReqwestClient>,
    sync_config: &BlueSkyPostSyncConfig,
) -> Result<Option<Union<RecordEmbedRefs>>> {
    #[allow(unused_assignments)]
    let temp_file_path = download_file_to_temp(&media_attachment.url, sync_config).await?;

    let mut should_fallback = false;

    // Check if the video exceeds the maximum duration (60 seconds) for BlueSky.
    let media_attachment_meta = media_attachment.meta.clone().unwrap();
    let video_duration = &media_attachment_meta.original.unwrap().duration.unwrap();
    if *video_duration >= MAX_VIDEO_DURATION {
        should_fallback = true;
    }

    // Check if the video exceeds the maximum size (50 MB) for BlueSky.
    let video_file_metadata = tokio::fs::metadata(&temp_file_path).await?;
    let video_file_size = video_file_metadata.len();
    if video_file_size >= MAX_VIDEO_SIZE {
        should_fallback = true;
    }

    let post_embed = match sync_config.config.bluesky_video_always_fallback || should_fallback {
        // Add a video link embed.
        true => {
            generate_video_link_embed(&media_attachment, mastodon_status, atp_client, sync_config)
                .await?
        }

        // Upload the video to BlueSky.
        false => {
            upload_video_to_bluesky(&media_attachment, &temp_file_path, atp_client, sync_config)
                .await?
        }
    };

    Ok(post_embed)
}

/// Generate a link embed for a video to add to a BlueSky post.
///
/// ## Arguments
///
/// * `media_attachment` - The video attachment to generate a link for.
/// * `mastodon_status` - The Mastodon status that the attachment originates from.
/// * `atp_client` - The client/agent for interacting with the AT Protocol.
/// * `sync_config` - Config for the sync.
async fn generate_video_link_embed(
    media_attachment: &megalodon::entities::attachment::Attachment,
    mastodon_status: &megalodon::entities::Status,
    atp_client: &AtpAgent<MemorySessionStore, ReqwestClient>,
    sync_config: &BlueSkyPostSyncConfig,
) -> Result<Option<Union<RecordEmbedRefs>>> {
    let temp_file_path =
        download_file_to_temp(&media_attachment.preview_url.clone().unwrap(), sync_config).await?;

    let video_link_thumbnail: bytes::Bytes = tokio::fs::read(&temp_file_path).await?.into();
    let video_link_thumbnail =
        ImageAttachmentData::new(video_link_thumbnail, &media_attachment.url)?;

    let blob_item = match video_link_thumbnail.image_bytes.len() > 0 {
        true => Some(
            atp_client
                .api
                .com
                .atproto
                .repo
                .upload_blob(video_link_thumbnail.image_bytes.into())
                .await?
                .blob
                .clone(),
        ),

        _ => None,
    };

    Ok(Some(Union::Refs(
        RecordEmbedRefs::AppBskyEmbedExternalMain(Box::new(
            app::bsky::embed::external::MainData {
                external: app::bsky::embed::external::ExternalData {
                    uri: mastodon_status.uri.clone(),
                    title: "View video on Mastodon".to_string(),
                    description: format!(
                        "Check out this video posted by @{}!",
                        mastodon_status.account.username.clone()
                    ),
                    thumb: blob_item,
                }
                .into(),
            }
            .into(),
        )),
    )))
}

/// Upload a video to BlueSky.
///
/// ## Arguments
///
/// * `media_attachment` - The video attachment to upload.
/// * `temp_path` - File path to temporarily write the data to.
/// * `atp_client` - The client/agent for interacting with the AT Protocol.
/// * `sync_config` - Config for the sync.
async fn upload_video_to_bluesky(
    media_attachment: &megalodon::entities::attachment::Attachment,
    temp_path: &std::path::PathBuf,
    atp_client: &AtpAgent<MemorySessionStore, ReqwestClient>,
    sync_config: &BlueSkyPostSyncConfig,
) -> Result<Option<Union<RecordEmbedRefs>>> {
    tracing::info!("Creating video upload service auth token");
    tracing::info!("{}", &sync_config.pds_service_endpoint);
    let service_auth_response = atp_client
        .api
        .com
        .atproto
        .server
        .get_service_auth(
            com::atproto::server::get_service_auth::ParametersData {
                aud: Did::new(format!("did:web:{}", &sync_config.pds_service_endpoint))
                    .map_err(|_| anyhow::anyhow!("Failed to create DID for video upload."))?,
                exp: Some((chrono::Utc::now() + chrono::Duration::minutes(30)).timestamp()),
                lxm: Some(
                    Nsid::new("com.atproto.repo.uploadBlob".to_string()).map_err(|_| {
                        anyhow::anyhow!("Failed to create NSID for com.atproto.repo.uploadBlob")
                    })?,
                ),
            }
            .into(),
        )
        .await?;

    let random_video_name = new_random_file_name(14, Some(".mp4"));

    // Upload the video to BlueSky.
    tracing::info!(
        "Uploading video attachment '{}' to Bluesky as '{}'",
        media_attachment.url,
        random_video_name
    );

    let temp_file = tokio::fs::File::open(temp_path).await?;

    let video_upload_client = create_http_client(&sync_config.config)?;
    let upload_video_job_response = video_upload_client
        .post("https://video.bsky.app/xrpc/app.bsky.video.uploadVideo")
        .query(&[
            ("did", sync_config.did.as_str()),
            ("name", &random_video_name),
        ])
        .bearer_auth(&service_auth_response.token)
        .header(CONTENT_TYPE, "video/mp4")
        .body(temp_file)
        .send()
        .await?
        .json::<JobStatus>()
        .await?;

    tracing::info!(
        "Waiting for video upload job '{}' to complete",
        upload_video_job_response.job_id
    );

    let mut job_status = upload_video_job_response.clone();

    while job_status.state != "JOB_STATE_FAILED" {
        job_status = video_upload_client
            .get("https://video.bsky.app/xrpc/app.bsky.video.getJobStatus")
            .query(&[("jobId", &job_status.job_id)])
            .send()
            .await?
            .json::<JobStatusResponse>()
            .await?
            .job_status
            .clone();

        if job_status.state == "JOB_STATE_COMPLETED" {
            break;
        }

        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }

    match job_status.state.as_str() {
        "JOB_STATE_FAILED" => {
            tracing::error!(
                "Failed to upload video attachment '{}'. Error message: '{}'",
                media_attachment.url,
                job_status
                    .error
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string())
            );

            return Err(FediProtoSyncError::VideoUploadError.into());
        }

        _ => {
            tracing::info!(
                "Uploaded video attachment '{}' to Bluesky",
                media_attachment.url
            );

            let blob = job_status.blob.clone().unwrap();

            return Ok(Some(Union::Refs(RecordEmbedRefs::AppBskyEmbedVideoMain(
                Box::new(
                    app::bsky::embed::video::MainData {
                        video: atrium_api::types::BlobRef::Typed(
                            atrium_api::types::TypedBlobRef::Blob(atrium_api::types::Blob {
                                r#ref: CidLink(ipld_core::cid::Cid::try_from(
                                    blob.item_ref.link.as_str(),
                                )?),
                                mime_type: blob.mime_type,
                                size: blob.size as usize,
                            }),
                        ),
                        alt: Some(
                            media_attachment
                                .description
                                .clone()
                                .unwrap_or_else(|| "".to_string()),
                        ),
                        aspect_ratio: None,
                        captions: None,
                    }
                    .into(),
                ),
            ))));
        }
    }
}

/// Download a file to a temporary directory.
///
/// ## Arguments
///
/// * `url` - The URL of the file to download.
/// * `sync_config` - Config for the sync.
async fn download_file_to_temp(
    url: &str,
    sync_config: &BlueSkyPostSyncConfig,
) -> Result<std::path::PathBuf> {
    let db_connection = &mut sync_config.db_connection_pool.get()?;

    let file_download_client = crate::core::create_http_client(&sync_config.config)?;
    let mut file_download_response = file_download_client.get(url).send().await?;

    let temp_path = std::env::temp_dir().join(new_random_file_name(14, None));
    let mut temp_file = tokio::fs::File::create(&temp_path).await?;

    while let Some(chunk) = file_download_response.chunk().await? {
        temp_file.write_all(&chunk).await?;
    }

    temp_file.flush().await?;

    let new_cached_file_record = NewCachedFile::new(&temp_path);
    fediproto_sync_db::operations::insert_cached_file_record(
        db_connection,
        &new_cached_file_record,
    )?;

    Ok(temp_path)
}

/// Generate richtext facets for tags/hashtags found in a Mastodon post.
///
/// ## Arguments
///
/// * `parsed_status` - The parsed Mastodon post.
fn generate_rich_text_tags(
    parsed_status: &ParsedMastodonPost
) -> Result<Vec<Object<app::bsky::richtext::facet::MainData>>> {
    let mut richtext_facets = Vec::<Object<app::bsky::richtext::facet::MainData>>::new();

    for tag in parsed_status.found_tags.clone() {
        // Find the start and end index of the tag in the post content to
        // generate a ByteSlice for the richtext facet.
        let tag_start_index = parsed_status.stripped_html.find(tag.as_str()).unwrap();
        let tag_end_index = tag_start_index + tag.len();

        let richtext_facet_tag = app::bsky::richtext::facet::MainData {
            index: app::bsky::richtext::facet::ByteSliceData {
                byte_start: tag_start_index,
                byte_end: tag_end_index,
            }
            .into(),
            features: vec![Union::Refs(
                app::bsky::richtext::facet::MainFeaturesItem::Tag(Box::new(
                    app::bsky::richtext::facet::Tag {
                        data: app::bsky::richtext::facet::TagData {
                            tag: tag.trim_start_matches("#").to_string(),
                        },
                        extra_data: Ipld::Null,
                    },
                )),
            )],
        };

        richtext_facets.push(richtext_facet_tag.into());
    }

    Ok(richtext_facets)
}

/// Generate richtext facets for links found in a Mastodon post.
///
/// ## Arguments
///
/// * `parsed_status` - The parsed Mastodon post.
fn generate_rich_text_links(
    parsed_status: &ParsedMastodonPost
) -> Result<Vec<Object<app::bsky::richtext::facet::MainData>>> {
    let mut richtext_facets = Vec::<Object<app::bsky::richtext::facet::MainData>>::new();

    for link in parsed_status.found_links.clone() {
        // Find the start and end index of the first link in the post content to
        // generate a ByteSlice for the richtext facet and add it to the list of
        // richtext facets for the post item.
        let link_start_index_filter = parsed_status
            .stripped_html
            .match_indices(&link)
            .filter(|(index, _)| {
                richtext_facets
                    .iter()
                    .any(|facet| facet.index.byte_start as usize != *index)
            })
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();

        let link_start_index = match link_start_index_filter.len() > 0 {
            true => link_start_index_filter[0],
            false => match parsed_status.stripped_html.find(&link) {
                Some(index) => index,
                None => return Err(anyhow::anyhow!("Link not found in post content")),
            },
        };

        let link_end_index = link_start_index + &link.len();

        let richtext_facet_link = app::bsky::richtext::facet::MainData {
            index: app::bsky::richtext::facet::ByteSliceData {
                byte_start: link_start_index,
                byte_end: link_end_index,
            }
            .into(),
            features: vec![Union::Refs(
                app::bsky::richtext::facet::MainFeaturesItem::Link(Box::new(
                    app::bsky::richtext::facet::Link {
                        data: app::bsky::richtext::facet::LinkData { uri: link.clone() },
                        extra_data: Ipld::Null,
                    },
                )),
            )],
        };

        richtext_facets.push(richtext_facet_link.into());
    }

    Ok(richtext_facets)
}

/// Generate a link embed for a BlueSky post.
///
/// ## Arguments
///
/// * `url` - The URL for the link embed.
/// * `atp_client` - The client/agent for interacting with the AT Protocol.
/// * `sync_config` - Config for the sync.
async fn generate_link_embed(
    url: &str,
    atp_client: &AtpAgent<MemorySessionStore, ReqwestClient>,
    sync_config: &BlueSkyPostSyncConfig,
) -> Result<Option<Union<app::bsky::feed::post::RecordEmbedRefs>>> {
    // Get metadata for the link.
    let link_metadata = get_link_metadata(url, sync_config).await?;

    // Get the thumbnail for the link if it has one and upload it to BlueSky.
    let link_thumbnail_url = link_metadata["image"].as_str().unwrap_or_else(|| "");
    let link_thumbnail = match link_thumbnail_url == "" {
        true => None,
        false => {
            let temp_file_path = download_file_to_temp(link_thumbnail_url, sync_config).await?;

            Some(ImageAttachmentData::new(
                tokio::fs::read(temp_file_path).await?.into(),
                link_thumbnail_url,
            )?)
        }
    };

    let blob_item = match link_thumbnail {
        Some(thumbnail) => Some(
            atp_client
                .api
                .com
                .atproto
                .repo
                .upload_blob(thumbnail.image_bytes.into())
                .await?
                .blob
                .clone(),
        ),
        _ => None,
    };

    let link_embed = Some(Union::Refs(
        app::bsky::feed::post::RecordEmbedRefs::AppBskyEmbedExternalMain(Box::new(
            app::bsky::embed::external::Main {
                data: app::bsky::embed::external::MainData {
                    external: app::bsky::embed::external::ExternalData {
                        uri: url.to_string(),
                        title: link_metadata["title"].as_str().unwrap().to_string(),
                        description: link_metadata["description"].as_str().unwrap().to_string(),
                        thumb: blob_item,
                    }
                    .into(),
                },
                extra_data: Ipld::Null,
            }
            .into(),
        )),
    ));

    Ok(link_embed)
}

/// Get link metadata using BlueSky's "CardyB" metadata service.
///
/// ## Arguments
///
/// * `url` - The URL to get metadata for.
/// * `sync_config` - Config for the sync.
async fn get_link_metadata(
    url: &str,
    sync_config: &BlueSkyPostSyncConfig,
) -> Result<serde_json::Value> {
    tracing::info!("Getting link metadata for '{}'.", url);
    let query_params = vec![("url", url)];

    let link_info_client = crate::core::create_http_client(&sync_config.config)?;

    let link_info_response = link_info_client
        .get("https://cardyb.bsky.app/v1/extract")
        .query(&query_params)
        .send()
        .await?;

    let link_info_json = link_info_response.json::<serde_json::Value>().await?;

    Ok(link_info_json)
}

/// Generate a link embed to a boosted Mastodon post.
///
/// ## Arguments
///
/// * `status` - The Mastodon post that has been boosted.
/// * `atp_client` - The client/agent for interacting with the AT Protocol.
/// * `sync_config` - Config for the sync.
async fn generate_boost_link_embed(
    status: &ParsedMastodonPost,
    atp_client: &AtpAgent<MemorySessionStore, ReqwestClient>,
    sync_config: &BlueSkyPostSyncConfig,
) -> Result<Option<Union<app::bsky::feed::post::RecordEmbedRefs>>> {
    // Get metadata for the link.
    let link_metadata = get_link_metadata(&status.mastodon_status.uri, sync_config).await?;

    // Get the thumbnail for the link if it has one and upload it to BlueSky.
    let link_thumbnail_url = link_metadata["image"].as_str().unwrap_or_else(|| "");
    let link_thumbnail = match link_thumbnail_url == "" {
        true => None,
        false => {
            let temp_file_path = download_file_to_temp(link_thumbnail_url, sync_config).await?;

            Some(ImageAttachmentData::new(
                tokio::fs::read(temp_file_path).await?.into(),
                link_thumbnail_url,
            )?)
        }
    };

    let blob_item = match link_thumbnail {
        Some(thumbnail) => Some(
            atp_client
                .api
                .com
                .atproto
                .repo
                .upload_blob(thumbnail.image_bytes.into())
                .await?
                .blob
                .clone(),
        ),
        _ => None,
    };

    let link_title = format!(
        "{} / 🚀 Boost",
        link_metadata["title"].as_str().unwrap().to_string()
    );

    Ok(Some(Union::Refs(
        app::bsky::feed::post::RecordEmbedRefs::AppBskyEmbedExternalMain(Box::new(
            app::bsky::embed::external::Main {
                data: app::bsky::embed::external::MainData {
                    external: app::bsky::embed::external::ExternalData {
                        uri: link_metadata["url"].as_str().unwrap().to_string(),
                        title: link_title,
                        description: status.stripped_html.clone(),
                        thumb: blob_item,
                    }
                    .into(),
                },
                extra_data: Ipld::Null,
            }
            .into(),
        )),
    )))
}

/// Resolve previously synced Mastodon posts.
///
/// ## Arguments
///
/// - `in_reply_to_id` - The ID of the previous post.
/// * `sync_config` - Config for the sync.
async fn resolve_previous_post(
    in_reply_to_id: &str,
    sync_config: &BlueSkyPostSyncConfig,
) -> Result<Option<(app::bsky::feed::post::ReplyRefData, String)>> {
    let db_connection = &mut sync_config.db_connection_pool.get()?;

    if !fediproto_sync_db::operations::check_synced_mastodon_post_exists(
        db_connection,
        in_reply_to_id,
    ) {
        return Ok(None);
    }

    let previous_mastodon_post = fediproto_sync_db::operations::get_synced_mastodon_post_by_id(
        db_connection,
        &in_reply_to_id,
    )?;

    let previous_synced_post = fediproto_sync_db::operations::get_bluesky_data_by_mastodon_post_id(
        db_connection,
        &in_reply_to_id,
    )?;

    let (previous_synced_post_root, previous_post_id) =
        match previous_mastodon_post.root_mastodon_post_id {
            Some(root_mastodon_post_id) => {
                // Set the previous post ID to the root post ID retrieved.
                (
                    fediproto_sync_db::operations::get_bluesky_data_by_mastodon_post_id(
                        db_connection,
                        &root_mastodon_post_id,
                    )?,
                    root_mastodon_post_id.clone(),
                )
            }

            None => {
                // Set the previous post ID to the previous post ID.
                (previous_synced_post.clone(), in_reply_to_id.to_string())
            }
        };

    let reply_ref = app::bsky::feed::post::ReplyRefData {
        root: com::atproto::repo::strong_ref::MainData {
            cid: Cid::from_str(&previous_synced_post_root.bsky_post_cid)?,
            uri: previous_synced_post_root.bsky_post_uri.clone(),
        }
        .into(),
        parent: com::atproto::repo::strong_ref::MainData {
            cid: Cid::from_str(&previous_synced_post.bsky_post_cid)?,
            uri: previous_synced_post.bsky_post_uri.clone(),
        }
        .into(),
    };

    Ok(Some((reply_ref, previous_post_id)))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlobItem {
    #[serde(rename = "$type")]
    pub item_type: String,

    #[serde(rename = "mimeType")]
    pub mime_type: String,

    #[serde(rename = "ref")]
    pub item_ref: BlobItemRef,

    #[serde(rename = "size")]
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlobItemRef {
    #[serde(rename = "$link")]
    pub link: String,
}

/// Represents the status of a video upload job.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct JobStatus {
    /// The ID of the job.
    #[serde(rename = "jobId")]
    pub job_id: String,

    /// The DID of the job.
    #[serde(rename = "did")]
    pub did: String,

    /// The state of the job.
    #[serde(rename = "state")]
    pub state: String,

    /// The progress of the job.
    #[serde(rename = "progress", default)]
    pub progress: i32,

    /// The blob of the job.
    #[serde(rename = "blob", skip_serializing_if = "Option::is_none")]
    pub blob: Option<BlobItem>,

    /// The error of the job.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// The message of the job.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// The response to a request for the status of a video upload job.
#[derive(Serialize, Deserialize, Debug)]
struct JobStatusResponse {
    /// The status of the job.
    #[serde(rename = "jobStatus")]
    pub job_status: JobStatus,
}

/// The response to a request to upload a video.
#[derive(Serialize, Deserialize, Debug)]
struct UploadVideoResponse {
    /// The status of the job.
    #[serde(rename = "jobStatus")]
    pub job_status: JobStatus,
}
