use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

/*
    app.bsky.feed.getLikes
*/

/*    Type: response
    Id: app.bsky.feed.getLikes#response
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Optional]
    - cursor: string (JsonProperty: cursor) [Optional]
    - likes: #like[] (JsonProperty: likes) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetLikesResponse {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "likes")]
    pub likes: Vec<Like>
}

/*    Type: like
    Id: app.bsky.feed.getLikes#like
    Kind: object

    Properties:
    - indexed_at: datetime (JsonProperty: indexedAt) [Required]
    - created_at: datetime (JsonProperty: createdAt) [Required]
    - actor: app.bsky.actor.defs#profileView (JsonProperty: actor) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Like {
    #[serde(rename = "indexedAt")]
    pub indexed_at: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "actor")]
    pub actor: ProfileView
}
