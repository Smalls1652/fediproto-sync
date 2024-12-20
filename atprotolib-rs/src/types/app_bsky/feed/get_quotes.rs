use serde::{Deserialize, Serialize};

use super::defs::PostView;

/*
    app.bsky.feed.getQuotes
*/

/*    Type: response
    Id: app.bsky.feed.getQuotes#response
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Optional]
    - cursor: string (JsonProperty: cursor) [Optional]
    - posts: app.bsky.feed.defs#postView[] (JsonProperty: posts) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetQuotesResponse {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "posts")]
    pub posts: Vec<PostView>
}
