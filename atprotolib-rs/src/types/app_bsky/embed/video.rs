use serde::{Deserialize, Serialize};

use super::AspectRatio;

/*
    app.bsky.embed.video
*/

/*    Type: caption
    Id: app.bsky.embed.video#caption
    Kind: object

    Properties:
    - lang: string (JsonProperty: lang) [Required]
    - file: blob  (JsonProperty: file) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.video#caption")]
pub struct VideoEmbedCaption {
    #[serde(rename = "lang")]
    pub lang: String,
    #[serde(rename = "file")]
    pub file: Vec<u8>
}

/*    Type: view
    Id: app.bsky.embed.video#view
    Kind: object

    Properties:
    - cid: string (JsonProperty: cid) [Required]
    - playlist: string (JsonProperty: playlist) [Required]
    - thumbnail: string (JsonProperty: thumbnail) [Optional]
    - alt: string (JsonProperty: alt) [Optional]
    - aspect_ratio: app.bsky.embed.defs#aspectRatio (JsonProperty: aspectRatio) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.video#view")]
pub struct VideoEmbedView {
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "playlist")]
    pub playlist: String,
    #[serde(rename = "thumbnail", skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(rename = "alt", skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(rename = "aspectRatio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>
}
