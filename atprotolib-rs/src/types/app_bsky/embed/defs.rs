use serde::{Deserialize, Serialize};

/// Represents the aspect ratio of an embed.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "$type", rename = "app.bsky.embed.defs#aspectRatio")]
pub struct AspectRatio {
    /// The width of the embed.
    #[serde(rename = "width", default)]
    pub width: i32,

    /// The height of the embed.
    #[serde(rename = "height", default)]
    pub height: i32
}
