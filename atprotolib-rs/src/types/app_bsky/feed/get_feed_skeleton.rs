use serde::{Deserialize, Serialize};

use super::defs::SkeletonFeedPost;

/*
    app.bsky.feed.getFeedSkeleton
*/

/*    Type: response
    Id: app.bsky.feed.getFeedSkeleton#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - feed: app.bsky.feed.defs#skeletonFeedPost[] (JsonProperty: feed) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetFeedSkeletonResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "feed")]
    pub feed: Vec<SkeletonFeedPost>
}
