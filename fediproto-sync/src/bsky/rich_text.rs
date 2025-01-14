use anyhow::Result;
use atrium_api::{
    self,
    app,
    types::{Object, Union}
};
use ipld_core::ipld::Ipld;

use super::{BlueSkyPostSync, BlueSkyPostSyncUtils};
use crate::{img_utils::ImageAttachmentData, mastodon::ParsedMastodonPost};

/// Trait for generating rich text facets for a BlueSky post.
pub trait BlueSkyPostSyncRichText {
    /// Generate rich text tags for the post item.
    ///
    /// ## Arguments
    ///
    /// * `parsed_status` - The parsed Mastodon post.
    fn generate_rich_text_tags(
        &self,
        parsed_status: &ParsedMastodonPost
    ) -> Result<Vec<Object<app::bsky::richtext::facet::MainData>>>;

    /// Generate rich text links for the post item.
    ///
    /// ## Arguments
    ///
    /// * `parsed_status` - The parsed Mastodon post.
    async fn generate_rich_text_links(
        &mut self,
        parsed_status: &ParsedMastodonPost
    ) -> Result<Vec<Object<app::bsky::richtext::facet::MainData>>>;

    /// Generate an embed for a link.
    ///
    /// ## Arguments
    ///
    /// * `url` - The URL of the link.
    /// * `is_boost` - Whether the link is a boosted Mastodon post.
    async fn generate_link_embed(
        &mut self,
        url: &str
    ) -> Result<()>;

    /// Generate an embed for a boosted post.
    ///
    /// ## Arguments
    ///
    /// * `status` - The boosted Mastodon post.
    async fn generate_boost_link_embed(
        &mut self,
        status: &ParsedMastodonPost
    ) -> Result<()>;
}

impl BlueSkyPostSyncRichText for BlueSkyPostSync<'_> {
    /// Generate rich text tags for the post item.
    ///
    /// ## Arguments
    ///
    /// * `parsed_status` - The parsed Mastodon post.
    fn generate_rich_text_tags(
        &self,
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
                    byte_end: tag_end_index
                }
                .into(),
                features: vec![Union::Refs(
                    app::bsky::richtext::facet::MainFeaturesItem::Tag(Box::new(
                        app::bsky::richtext::facet::Tag {
                            data: app::bsky::richtext::facet::TagData {
                                tag: tag.trim_start_matches("#").to_string()
                            },
                            extra_data: Ipld::Null
                        }
                    ))
                )]
            };

            richtext_facets.push(richtext_facet_tag.into());
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
                    None => return Err(anyhow::anyhow!("Link not found in post content"))
                }
            };

            let link_end_index = link_start_index + &link.len();

            let richtext_facet_link = app::bsky::richtext::facet::MainData {
                index: app::bsky::richtext::facet::ByteSliceData {
                    byte_start: link_start_index,
                    byte_end: link_end_index
                }
                .into(),
                features: vec![Union::Refs(
                    app::bsky::richtext::facet::MainFeaturesItem::Link(Box::new(
                        app::bsky::richtext::facet::Link {
                            data: app::bsky::richtext::facet::LinkData { uri: link.clone() },
                            extra_data: Ipld::Null
                        }
                    ))
                )]
            };

            richtext_facets.push(richtext_facet_link.into());
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

            self.generate_link_embed(&first_link).await?;
        }

        Ok(richtext_facets)
    }

    /// Generate an embed for a link.
    ///
    /// ## Arguments
    ///
    /// * `url` - The URL of the link.
    async fn generate_link_embed(
        &mut self,
        url: &str
    ) -> Result<()> {
        // Get metadata for the link.
        let link_metadata = self.get_link_metadata(url).await?;

        // Get the thumbnail for the link if it has one and upload it to BlueSky.
        let link_thumbnail_url = link_metadata["image"].as_str().unwrap_or_else(|| "");
        let link_thumbnail = match link_thumbnail_url == "" {
            true => None,
            false => {
                let temp_file_path = self.download_file_to_temp(link_thumbnail_url).await?;

                Some(ImageAttachmentData::new(
                    tokio::fs::read(temp_file_path).await?.into(),
                    link_thumbnail_url
                )?)
            }
        };

        let blob_item = match link_thumbnail {
            Some(thumbnail) => Some(
                self.atp_agent
                    .api
                    .com
                    .atproto
                    .repo
                    .upload_blob(thumbnail.image_bytes.into())
                    .await?
                    .blob
                    .clone()
            ),
            _ => None
        };

        self.post_item.embed = Some(Union::Refs(
            app::bsky::feed::post::RecordEmbedRefs::AppBskyEmbedExternalMain(Box::new(
                app::bsky::embed::external::Main {
                    data: app::bsky::embed::external::MainData {
                        external: app::bsky::embed::external::ExternalData {
                            uri: url.to_string(),
                            title: link_metadata["title"].as_str().unwrap().to_string(),
                            description: link_metadata["description"].as_str().unwrap().to_string(),
                            thumb: blob_item
                        }
                        .into()
                    },
                    extra_data: Ipld::Null
                }
                .into()
            ))
        ));

        Ok(())
    }

    /// Generate an embed for a boosted post.
    ///
    /// ## Arguments
    ///
    /// * `status` - The boosted Mastodon post.
    async fn generate_boost_link_embed(
        &mut self,
        status: &ParsedMastodonPost
    ) -> Result<()> {
        // Get metadata for the link.
        let link_metadata = self.get_link_metadata(&status.mastodon_status.uri).await?;

        // Get the thumbnail for the link if it has one and upload it to BlueSky.
        let link_thumbnail_url = link_metadata["image"].as_str().unwrap_or_else(|| "");
        let link_thumbnail = match link_thumbnail_url == "" {
            true => None,
            false => {
                let temp_file_path = self.download_file_to_temp(link_thumbnail_url).await?;

                Some(ImageAttachmentData::new(
                    tokio::fs::read(temp_file_path).await?.into(),
                    link_thumbnail_url
                )?)
            }
        };

        let blob_item = match link_thumbnail {
            Some(thumbnail) => Some(
                self.atp_agent
                    .api
                    .com
                    .atproto
                    .repo
                    .upload_blob(thumbnail.image_bytes.into())
                    .await?
                    .blob
                    .clone()
            ),
            _ => None
        };

        let link_title = format!(
            "{} / 🚀 Boost",
            link_metadata["title"].as_str().unwrap().to_string()
        );

        self.post_item.embed = Some(Union::Refs(
            app::bsky::feed::post::RecordEmbedRefs::AppBskyEmbedExternalMain(Box::new(
                app::bsky::embed::external::Main {
                    data: app::bsky::embed::external::MainData {
                        external: app::bsky::embed::external::ExternalData {
                            uri: link_metadata["url"].as_str().unwrap().to_string(),
                            title: link_title,
                            description: status.stripped_html.clone(),
                            thumb: blob_item
                        }
                        .into()
                    },
                    extra_data: Ipld::Null
                }
                .into()
            ))
        ));

        Ok(())
    }
}
