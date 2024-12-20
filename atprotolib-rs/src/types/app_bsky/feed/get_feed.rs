use serde::{Deserialize, Serialize};

use super::defs::FeedViewPost;

/*
    app.bsky.feed.getFeed
*/

/*    Type: response
    Id: app.bsky.feed.getFeed#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - feed: app.bsky.feed.defs#feedViewPost[] (JsonProperty: feed) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetFeedResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "feed")]
    pub feed: Vec<FeedViewPost>
}
