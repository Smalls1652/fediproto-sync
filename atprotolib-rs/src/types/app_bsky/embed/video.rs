use serde::{Deserialize, Serialize};

use super::AspectRatio;

/// Represents captions for a video embed.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.video#caption")]
pub struct VideoEmbedCaption {
    /// The language of the captions.
    #[serde(rename = "lang")]
    pub lang: String,

    /// The file containing the captions.
    #[serde(rename = "file")]
    pub file: Vec<u8>
}

/// Represents a video embed view.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.video#view")]
pub struct VideoEmbedView {
    /// The content identifier of the video.
    #[serde(rename = "cid")]
    pub cid: String,

    /// The playlist of the video.
    #[serde(rename = "playlist")]
    pub playlist: String,

    /// A thumbnail image representing the video.
    #[serde(rename = "thumbnail", skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,

    /// Accessibility text for the video.
    #[serde(rename = "alt", skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,

    /// The aspect ratio of the video.
    #[serde(rename = "aspectRatio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>
}
