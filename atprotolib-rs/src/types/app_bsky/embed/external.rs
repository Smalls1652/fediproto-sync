use serde::{Deserialize, Serialize};

use crate::types::com_atproto::repo::BlobItem;

/// A representation of some externally linked content (eg, a URL and 'card'), embedded in a Bluesky record (eg, a post).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalEmbed {
    /// The URI of the external content.
    #[serde(rename = "uri")]
    pub uri: String,

    /// The title of the external content.
    #[serde(rename = "title")]
    pub title: String,

    /// A description of the external content.
    #[serde(rename = "description")]
    pub description: String,

    /// A thumbnail image representing the external content.
    #[serde(rename = "thumb", skip_serializing_if = "Option::is_none")]
    pub thumb: Option<BlobItem>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalEmbedThumbnail {
    #[serde(rename = "$type")]
    pub embed_type: String,

    #[serde(rename = "mimeType")]
    pub mime_type: String,

    #[serde(rename = "ref")]
    pub thumbnail_ref: ExternalEmbedThumbnailRef,

    #[serde(rename = "size")]
    pub size: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalEmbedThumbnailRef {
    #[serde(rename = "$link")]
    pub link: String
}

/// A view of an external embed.
#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalEmbedView {
    /// The external embed.
    #[serde(rename = "external")]
    pub external: ExternalEmbed
}

/// A view of an external embed, with the external embed itself embedded. (?)
#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalEmbedViewExternal {
    /// The URI of the external content.
    #[serde(rename = "uri")]
    pub uri: String,

    /// The title of the external content.
    #[serde(rename = "title")]
    pub title: String,

    /// A description of the external content.
    #[serde(rename = "description")]
    pub description: String,

    /// A URI to a thumbnail image representing the external content.
    #[serde(rename = "thumb", skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>
}
