use atprotolib_rs::types::com_atproto;

use super::BlueSkyPostSync;

pub trait BlueSkyPostSyncUtils {
    /// Get link metadata from the CardyB API.
    ///
    /// ## Arguments
    ///
    /// * `url` - The URL to get metadata for.
    async fn get_link_metadata(
        &mut self,
        url: &str
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>;

    /// Get a link thumbnail returned by the CardyB API.
    ///
    /// ## Arguments
    ///
    /// * `image_url` - The URL of the image to get.
    async fn get_link_thumbnail(
        &mut self,
        image_url: &str
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>>;

    /// Get the PDS service endpoint from the Bluesky session.
    fn get_pds_service_endpoint(&mut self) -> Result<String, Box<dyn std::error::Error>>;
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
