use serde::{Deserialize, Serialize};

use crate::types::com_atproto::repo::BlobItem;

use super::AspectRatio;

/*
    app.bsky.embed.images
*/

/// A representation of an image embedded in a Bluesky record (eg, a post).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageEmbed {
    /// The image data.
    #[serde(rename = "image")]
    pub image: BlobItem,

    /// Alt text for the image.
    #[serde(rename = "alt")]
    pub alt: String,

    /// The aspect ratio of the image.
    #[serde(rename = "aspectRatio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageEmbedImage {
    #[serde(rename = "$type")]
    pub embed_type: String,

    #[serde(rename = "mimeType")]
    pub mime_type: String,

    #[serde(rename = "ref")]
    pub image_ref: ImageEmbedImageRef,

    #[serde(rename = "size")]
    pub size: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageEmbedImageRef {
    #[serde(rename = "$link")]
    pub link: String
}

/// A view of an image embed.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageEmbedView {
    /// A list of image embeds.
    #[serde(rename = "images")]
    pub images: Vec<ImageEmbed>
}

/// A view of an image embed, with the image embed itself embedded. (?)
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageEmbedViewImage {
    /// Fully-qualified URL where a thumbnail of the image can be fetched. For
    /// example, CDN location provided by the App View.
    #[serde(rename = "thumb")]
    pub thumb: String,

    /// Fully-qualified URL where a large version of the image can be fetched.
    /// May or may not be the exact original blob. For example, CDN location
    /// provided by the App View.
    #[serde(rename = "fullsize")]
    pub fullsize: String,

    /// Alt text for the image.
    #[serde(rename = "alt")]
    pub alt: String,

    /// The aspect ratio of the image.
    #[serde(rename = "aspectRatio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>
}
