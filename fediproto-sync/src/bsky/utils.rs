use anyhow::Result;
use fediproto_sync_db::models::NewCachedFile;
use rand::distributions::DistString;
use tokio::io::AsyncWriteExt;

use super::BlueSkyPostSync;

/// Trait for utility functions used in BlueSky post synchronization.
pub trait BlueSkyPostSyncUtils {
    /// Get link metadata from the CardyB API.
    ///
    /// ## Arguments
    ///
    /// * `url` - The URL to get metadata for.
    async fn get_link_metadata(
        &mut self,
        url: &str
    ) -> Result<serde_json::Value>;

    /// Get a link thumbnail returned by the CardyB API.
    ///
    /// ## Arguments
    ///
    /// * `image_url` - The URL of the image to get.
    #[allow(dead_code)]
    async fn get_link_thumbnail(
        &mut self,
        image_url: &str
    ) -> Result<reqwest::Response>;

    /// Download a file to a temporary location.
    ///
    /// ## Arguments
    ///
    /// * `url` - The URL of the file to download.
    async fn download_file_to_temp(
        &mut self,
        url: &str
    ) -> Result<std::path::PathBuf>;
}

impl BlueSkyPostSyncUtils for BlueSkyPostSync<'_> {
    /// Get link metadata from the CardyB API.
    ///
    /// ## Arguments
    ///
    /// * `url` - The URL to get metadata for.
    async fn get_link_metadata(
        &mut self,
        url: &str
    ) -> Result<serde_json::Value> {
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
    ) -> Result<reqwest::Response> {
        tracing::info!("Getting link thumbnail for '{}'.", image_url);

        let link_thumbnail_client = crate::core::create_http_client(&self.config)?;

        let link_thumbnail_response = link_thumbnail_client.get(image_url).send().await?;

        Ok(link_thumbnail_response)
    }

    /// Download a file to a temporary location.
    ///
    /// ## Arguments
    ///
    /// * `url` - The URL of the file to download.
    async fn download_file_to_temp(
        &mut self,
        url: &str
    ) -> Result<std::path::PathBuf> {
        let db_connection = &mut self.db_connection_pool.get()?;

        let file_download_client = crate::core::create_http_client(&self.config)?;
        let mut file_download_response = file_download_client.get(url).send().await?;

        let temp_path = std::env::temp_dir()
            .join(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 14));
        let mut temp_file = tokio::fs::File::create(&temp_path).await?;

        while let Some(chunk) = file_download_response.chunk().await? {
            temp_file.write_all(&chunk).await?;
        }

        temp_file.flush().await?;

        let new_cached_file_record = NewCachedFile::new(&temp_path);
        fediproto_sync_db::operations::insert_cached_file_record(
            db_connection,
            &new_cached_file_record
        )?;

        Ok(temp_path)
    }
}
