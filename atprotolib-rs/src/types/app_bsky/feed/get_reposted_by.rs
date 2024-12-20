use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

/*
    app.bsky.feed.getRepostedBy
*/

/*    Type: response
    Id: app.bsky.feed.getRepostedBy#response
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Optional]
    - cursor: string (JsonProperty: cursor) [Optional]
    - reposted_by: app.bsky.actor.defs#profileView[] (JsonProperty: repostedBy) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetRepostedByResponse {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "reposted_by")]
    pub reposted_by: Vec<ProfileView>
}
