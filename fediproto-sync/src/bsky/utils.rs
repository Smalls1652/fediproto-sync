use anyhow::Result;

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
    async fn get_link_thumbnail(
        &mut self,
        image_url: &str
    ) -> Result<reqwest::Response>;
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
}
