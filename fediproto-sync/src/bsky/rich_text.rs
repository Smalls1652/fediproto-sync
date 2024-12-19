use atprotolib_rs::types::{
    app_bsky::{self, richtext::RichTextFacet},
    com_atproto
};
use bytes::Bytes;

use super::BlueSkyPostSync;
use crate::bsky::{media::MAX_IMAGE_SIZE, utils::BlueSkyPostSyncUtils};

/// Trait for generating rich text facets for a BlueSky post.
pub trait BlueSkyPostSyncRichText {
    /// Generate rich text tags for the post item.
    ///
    /// ## Arguments
    ///
    /// * `parsed_status` - The parsed Mastodon post.
    fn generate_rich_text_tags(
        &self,
        parsed_status: &crate::mastodon::ParsedMastodonPost
    ) -> Result<Vec<RichTextFacet>, Box<dyn std::error::Error>>;

    /// Generate rich text links for the post item.
    ///
    /// ## Arguments
    ///
    /// * `parsed_status` - The parsed Mastodon post.
    async fn generate_rich_text_links(
        &mut self,
        parsed_status: &crate::mastodon::ParsedMastodonPost
    ) -> Result<Vec<RichTextFacet>, Box<dyn std::error::Error>>;
}

impl BlueSkyPostSyncRichText for BlueSkyPostSync {
    /// Generate rich text tags for the post item.
    ///
    /// ## Arguments
    ///
    /// * `parsed_status` - The parsed Mastodon post.
    fn generate_rich_text_tags(
        &self,
        parsed_status: &crate::mastodon::ParsedMastodonPost
    ) -> Result<Vec<RichTextFacet>, Box<dyn std::error::Error>> {
        let mut richtext_facets = Vec::<RichTextFacet>::new();

        for tag in parsed_status.found_tags.clone() {
            // Find the start and end index of the tag in the post content to
            // generate a ByteSlice for the richtext facet.
            let tag_start_index = parsed_status.stripped_html.find(tag.as_str()).unwrap();
            let tag_end_index = tag_start_index + tag.len();

            // Create a richtext facet for the tag and add it to the list of richtext
            // facets.
            let richtext_facet_tag = RichTextFacet {
                index: app_bsky::richtext::ByteSlice {
                    byte_start: tag_start_index as i64,
                    byte_end: tag_end_index as i64
                },
                features: vec![app_bsky::richtext::RichTextFacetFeature::Tag(
                    app_bsky::richtext::RichTextFacetTag {
                        tag: tag.trim_start_matches("#").to_string()
                    }
                )]
            };

            richtext_facets.push(richtext_facet_tag);
        }

        Ok(richtext_facets)
    }

    /// Generate rich text links for the post item.
    ///
    /// ## Arguments
    ///
    /// * `parsed_status` - The parsed Mastodon post.
    async fn generate_rich_text_links(
        &mut self,
        parsed_status: &crate::mastodon::ParsedMastodonPost
    ) -> Result<Vec<RichTextFacet>, Box<dyn std::error::Error>> {
        let mut richtext_facets = Vec::<RichTextFacet>::new();

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
                    None => return Err("Link not found in post content".into())
                }
            };

            let link_end_index = link_start_index + &link.len();

            let richtext_facet_link = RichTextFacet {
                index: app_bsky::richtext::ByteSlice {
                    byte_start: link_start_index as i64,
                    byte_end: link_end_index as i64
                },
                features: vec![app_bsky::richtext::RichTextFacetFeature::Link(
                    app_bsky::richtext::RichTextFacetLink { uri: link.clone() }
                )]
            };

            richtext_facets.push(richtext_facet_link);
        }

        // Check if the post has an embed and add an external embed for the first link
        // if it doesn't.
        if self.post_item.embed.is_none() {
            // Get the first link found in the post.
            let first_link = parsed_status.found_links[0].clone();

            tracing::info!(
                "Post has no embeds, adding external embed for link '{}'",
                first_link
            );

            // Get metadata for the link.
            let link_metadata = self.get_link_metadata(&first_link).await?;

            // Get the thumbnail for the link if it has one and upload it to BlueSky.
            let link_thumbnail_url = link_metadata["image"].as_str().unwrap_or_else(|| "");
            let link_thumbnail_bytes = match link_thumbnail_url == "" {
                true => Bytes::new(),
                false => {
                    let link_thumbnail = self.get_link_thumbnail(link_thumbnail_url).await?;

                    link_thumbnail.bytes().await?
                }
            };

            let link_thumbnail_bytes = match link_thumbnail_bytes.len() > MAX_IMAGE_SIZE as usize {
                true => {
                    let compressed_image = crate::img_utils::compress_image(link_thumbnail_bytes.as_ref())?;

                    tracing::info!(
                        "Compressed link thumbnail from {} bytes to {} bytes",
                        link_thumbnail_bytes.len(),
                        compressed_image.len()
                    );

                    compressed_image
                }

                _ => link_thumbnail_bytes
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
            self.post_item.embed = Some(app_bsky::feed::PostEmbeds::External(
                app_bsky::feed::PostEmbedExternal {
                    external: app_bsky::embed::ExternalEmbed {
                        uri: link_metadata["url"].as_str().unwrap().to_string(),
                        title: link_metadata["title"].as_str().unwrap().to_string(),
                        description: link_metadata["description"].as_str().unwrap().to_string(),
                        thumb: blob_item
                    }
                }
            ));
        }

        Ok(richtext_facets)
    }
}
