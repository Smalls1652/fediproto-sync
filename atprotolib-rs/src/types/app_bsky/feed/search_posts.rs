use serde::{Deserialize, Serialize};

use super::defs::PostView;

/*
    app.bsky.feed.searchPosts
*/

/*    Type: response
    Id: app.bsky.feed.searchPosts#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - hits_total: integer  (JsonProperty: hitsTotal) [Optional]
    - posts: app.bsky.feed.defs#postView[] (JsonProperty: posts) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchPostsResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "hitsTotal", default)]
    pub hits_total: i32,
    #[serde(rename = "posts")]
    pub posts: Vec<PostView>
}
