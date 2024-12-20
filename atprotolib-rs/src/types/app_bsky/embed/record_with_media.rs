use serde::{Deserialize, Serialize};

use super::{
    external::ExternalEmbedView,
    image::ImageEmbedView,
    record::RecordEmbedView,
    video::VideoEmbedView
};

/// A representation of a record with media embedded in a Bluesky record (eg, a
/// post).
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.recordWithMedia#view")]
pub struct RecordWithMediaEmbedView {
    /// The embedded record.
    #[serde(rename = "record")]
    pub record: RecordEmbedView,

    /// The media embedded in the record.
    #[serde(rename = "media")]
    pub media: RecordWithMediaEmbedViewMedia
}

/// Represents the media embedded in a record with media.
#[derive(Serialize, Deserialize, Debug)]
pub enum RecordWithMediaEmbedViewMedia {
    /// Images embedded in the record.
    Images(ImageEmbedView),

    /// Videos embedded in the record.
    Videos(VideoEmbedView),

    /// External content embedded in the record.
    External(ExternalEmbedView)
}
