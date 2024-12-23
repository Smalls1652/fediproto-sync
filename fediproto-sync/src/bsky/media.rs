use atrium_api::{
    self,
    app::{
        self,
        bsky::{embed::images::ImageData, feed::post::RecordEmbedRefs}
    },
    com,
    types::{
        string::{Did, Nsid},
        Object,
        Union
    }
};
use fediproto_sync_db::models::NewCachedFile;
use fediproto_sync_lib::error::{FediProtoSyncError, FediProtoSyncErrorKind};
use ipld_core::ipld::Ipld;
use rand::distributions::DistString;
use tokio::io::AsyncWriteExt;

use super::{BlueSkyPostSync, BlueSkyPostSyncUtils};
use crate::core::create_atp_service_client;

/// The maximum duration for a BlueSky video in seconds.
///
/// (Currently `60` seconds)
pub const MAX_VIDEO_DURATION: f64 = 60.0;

/// The maximum size for a BlueSky image in bytes.
///
/// (Currently `976.56 KB`, but set to `950 KB` to account for overhead)
pub const MAX_IMAGE_SIZE: u64 = 950_000;

/// The maximum size for a BlueSky video in bytes.
///
/// (Currently `50 MB`)
pub const MAX_VIDEO_SIZE: u64 = 50_000_000;

/// Trait for generating media embeds for a BlueSky post.
pub trait BlueSkyPostSyncMedia {
    /// Generate an image embed for a BlueSky post from media attachments from
    /// a Mastodon status.
    ///
    /// ## Arguments
    ///
    /// * `media_attachments` - The media attachments to generate the image
    ///   embed
    async fn generate_image_embed(
        &mut self,
        media_attachments: &Vec<megalodon::entities::attachment::Attachment>
    ) -> Result<Option<Union<RecordEmbedRefs>>, Box<dyn std::error::Error>>;

    /// Generate a video embed for a BlueSky post from a media attachment from
    /// a Mastodon status.
    ///
    /// ## Arguments
    ///
    /// * `media_attachment` - The media attachment to generate the video embed
    ///   from.
    async fn generate_video_embed(
        &mut self,
        media_attachment: &megalodon::entities::attachment::Attachment
    ) -> Result<Option<Union<RecordEmbedRefs>>, Box<dyn std::error::Error>>;

    /// Create a video link embed for a BlueSky post from a media attachment
    /// from a Mastodon status.
    ///
    /// ## Arguments
    ///
    /// * `media_attachment` - The media attachment to create the video link
    ///   embed.
    async fn generate_video_link_embed(
        &mut self,
        media_attachment: &megalodon::entities::attachment::Attachment
    ) -> Result<Option<Union<RecordEmbedRefs>>, Box<dyn std::error::Error>>;

    /// Upload a video attachment to BlueSky.
    ///
    /// ## Arguments
    ///
    /// * `media_attachment` - The media attachment to upload to BlueSky.
    /// * `file_path` - The path to the video file to upload.
    async fn upload_video_to_bluesky(
        &mut self,
        media_attachment: &megalodon::entities::attachment::Attachment,
        file_path: &std::path::PathBuf
    ) -> Result<Option<Union<RecordEmbedRefs>>, Box<dyn std::error::Error>>;

    /// Download a media attachment from a Mastodon status.
    ///
    /// ## Arguments
    ///
    /// * `media_attachment` - The media attachment to download.
    async fn download_mastodon_media_attachment(
        &mut self,
        media_attachment: &megalodon::entities::attachment::Attachment
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>>;

    /// Download a media attachment from a Mastodon status to a temporary file.
    ///
    /// ## Arguments
    ///
    /// * `media_attachment` - The media attachment to download.
    async fn download_mastodon_media_attachment_to_file(
        &mut self,
        media_attachment: &megalodon::entities::attachment::Attachment
    ) -> Result<std::path::PathBuf, Box<dyn std::error::Error>>;
}

impl BlueSkyPostSyncMedia for BlueSkyPostSync<'_> {
    /// Generate an image embed for a BlueSky post from media attachments from
    /// a Mastodon status.
    ///
    /// ## Arguments
    ///
    /// * `media_attachments` - The media attachments to generate the image
    ///   embed
    async fn generate_image_embed(
        &mut self,
        media_attachments: &Vec<megalodon::entities::attachment::Attachment>
    ) -> Result<Option<Union<RecordEmbedRefs>>, Box<dyn std::error::Error>> {
        let mut image_attachments = Vec::<Object<ImageData>>::new();

        for image_attachment in media_attachments {
            // Download the media attachment from the Mastodon server.
            let media_attachment_temp_path = self
                .download_mastodon_media_attachment_to_file(image_attachment)
                .await?;

            let media_attachment_bytes = tokio::fs::read(&media_attachment_temp_path).await?;

            let blob_upload_response = self
                .atp_agent
                .api
                .com
                .atproto
                .repo
                .upload_blob(media_attachment_bytes)
                .await?;

            tokio::fs::remove_file(&media_attachment_temp_path).await?;

            // Create an image embed and add it to the list of image attachments.
            image_attachments.push(
                app::bsky::embed::images::ImageData {
                    image: blob_upload_response.blob.clone(),
                    alt: image_attachment
                        .description
                        .clone()
                        .unwrap_or_else(|| "".to_string()),
                    aspect_ratio: None
                }
                .into()
            );

            tracing::info!(
                "Uploaded media attachment '{}' to Bluesky",
                image_attachment.url
            );
        }

        Ok(Some(Union::Refs(RecordEmbedRefs::AppBskyEmbedImagesMain(
            Box::new(
                app::bsky::embed::images::MainData {
                    images: image_attachments
                }
                .into()
            )
        ))))
    }

    /// Generate a video embed for a BlueSky post from a media attachment from
    /// a Mastodon status.
    ///
    /// ## Arguments
    ///
    /// * `media_attachment` - The media attachment to generate the video embed
    ///   from.
    async fn generate_video_embed(
        &mut self,
        media_attachment: &megalodon::entities::attachment::Attachment
    ) -> Result<Option<Union<RecordEmbedRefs>>, Box<dyn std::error::Error>> {
        let db_connection = &mut self.db_connection_pool.get()?;

        #[allow(unused_assignments)]
        let temp_file_path = self
            .download_mastodon_media_attachment_to_file(media_attachment)
            .await?;

        let new_cached_file_record = NewCachedFile::new(&temp_file_path);
        fediproto_sync_db::operations::insert_cached_file_record(
            db_connection,
            &new_cached_file_record
        )?;

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

        let post_embed = match self.config.bluesky_video_always_fallback || should_fallback {
            // Add a video link embed.
            true => self.generate_video_link_embed(&media_attachment).await?,

            // Upload the video to BlueSky.
            false => {
                self.upload_video_to_bluesky(&media_attachment, &temp_file_path)
                    .await?
            }
        };

        Ok(post_embed)
    }

    /// Create a video link embed for a BlueSky post from a media attachment
    /// from a Mastodon status.
    ///
    /// ## Arguments
    ///
    /// * `media_attachment` - The media attachment to create the video link
    async fn generate_video_link_embed(
        &mut self,
        media_attachment: &megalodon::entities::attachment::Attachment
    ) -> Result<Option<Union<RecordEmbedRefs>>, Box<dyn std::error::Error>> {
        let video_link_thumbnail_bytes = self
            .get_link_thumbnail(media_attachment.preview_url.clone().unwrap().as_str())
            .await?;
        let video_link_thumbnail_bytes = video_link_thumbnail_bytes.bytes().await?;
        let video_link_thumbnail_bytes =
            match video_link_thumbnail_bytes.len() > MAX_IMAGE_SIZE as usize {
                true => {
                    let compressed_image =
                        crate::img_utils::compress_image(&video_link_thumbnail_bytes)?;

                    tracing::info!(
                        "Compressed video link thumbnail from {} bytes to {} bytes",
                        video_link_thumbnail_bytes.len(),
                        compressed_image.len()
                    );

                    compressed_image
                }

                _ => video_link_thumbnail_bytes
            };

        let blob_item = match video_link_thumbnail_bytes.len() > 0 {
            true => Some(
                self.atp_agent
                    .api
                    .com
                    .atproto
                    .repo
                    .upload_blob(video_link_thumbnail_bytes.to_vec())
                    .await?
                    .blob
                    .clone()
            ),

            _ => None
        };

        Ok(Some(Union::Refs(
            RecordEmbedRefs::AppBskyEmbedExternalMain(Box::new(
                app::bsky::embed::external::MainData {
                    external: app::bsky::embed::external::ExternalData {
                        uri: media_attachment.url.clone(),
                        title: "View video on Mastodon".to_string(),
                        description: format!(
                            "Check out this video posted by @{}!",
                            self.mastodon_status.account.username.clone()
                        ),
                        thumb: blob_item
                    }
                    .into()
                }
                .into()
            ))
        )))
    }

    /// Upload a video attachment to BlueSky.
    ///
    /// ## Arguments
    ///
    /// * `media_attachment` - The media attachment to upload to BlueSky.
    async fn upload_video_to_bluesky(
        &mut self,
        media_attachment: &megalodon::entities::attachment::Attachment,
        temp_path: &std::path::PathBuf
    ) -> Result<Option<Union<RecordEmbedRefs>>, Box<dyn std::error::Error>> {
        let service_auth_response = self
            .atp_agent
            .api
            .com
            .atproto
            .server
            .get_service_auth(
                com::atproto::server::get_service_auth::ParametersData {
                    aud: Did::new(format!("did:web:{}", self.pds_service_endpoint))?,
                    exp: Some((chrono::Utc::now() + chrono::Duration::minutes(30)).timestamp()),
                    lxm: Some(Nsid::new("com.atproto.repo.uploadBlob".to_string())?)
                }
                .into()
            )
            .await?;

        let video_upload_client = create_atp_service_client(
            "video.bsky.app",
            Some(&service_auth_response.token),
            &self.config
        )?;

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

        let temp_file = tokio::fs::read(temp_path).await?;

        let upload_video_job_response = video_upload_client
            .service
            .app
            .bsky
            .video
            .upload_video(temp_file)
            .await?;

        tracing::info!(
            "Waiting for video upload job '{}' to complete",
            upload_video_job_response.job_status.job_id
        );

        let job_client = create_atp_service_client("video.bsky.app", None, &self.config)?;

        let mut job_status = upload_video_job_response.job_status.clone();

        while job_status.state != "JOB_STATE_FAILED" {
            job_status = job_client
                .service
                .app
                .bsky
                .video
                .get_job_status(app::bsky::video::get_job_status::Parameters {
                    data: app::bsky::video::get_job_status::ParametersData {
                        job_id: job_status.job_id.clone()
                    },
                    extra_data: Ipld::Null
                })
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

                return Err(Box::new(FediProtoSyncError::new(
                    "The BlueSky upload job failed.",
                    FediProtoSyncErrorKind::VideoUploadError
                )));
            }

            _ => {
                tracing::info!(
                    "Uploaded video attachment '{}' to Bluesky",
                    media_attachment.url
                );

                return Ok(Some(Union::Refs(RecordEmbedRefs::AppBskyEmbedVideoMain(
                    Box::new(
                        app::bsky::embed::video::MainData {
                            video: job_status.blob.clone().unwrap(),
                            alt: Some(
                                media_attachment
                                    .description
                                    .clone()
                                    .unwrap_or_else(|| "".to_string())
                            ),
                            aspect_ratio: None,
                            captions: None
                        }
                        .into()
                    )
                ))));
            }
        }
    }

    /// Download a media attachment from a Mastodon status.
    ///
    /// ## Arguments
    ///
    /// * `media_attachment` - The media attachment to download.
    async fn download_mastodon_media_attachment(
        &mut self,
        media_attachment: &megalodon::entities::attachment::Attachment
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        tracing::info!(
            "Downloading media attachment '{}' from Mastodon",
            media_attachment.url
        );

        let media_attachment_client = crate::core::create_http_client(&self.config)?;
        let media_attachment_response = media_attachment_client
            .get(&media_attachment.url)
            .send()
            .await?;

        Ok(media_attachment_response)
    }

    /// Download a media attachment from a Mastodon status to a temporary file.
    ///
    /// ## Arguments
    ///
    /// * `media_attachment` - The media attachment to download.
    async fn download_mastodon_media_attachment_to_file(
        &mut self,
        media_attachment: &megalodon::entities::attachment::Attachment
    ) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let mut media_attachment_response = self
            .download_mastodon_media_attachment(media_attachment)
            .await?;

        let temp_path = std::env::temp_dir()
            .join(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 14));
        let mut temp_file = tokio::fs::File::create(&temp_path).await?;

        while let Some(chunk) = media_attachment_response.chunk().await? {
            temp_file.write_all(&chunk).await?;
        }

        temp_file.flush().await?;

        Ok(temp_path)
    }
}
