use serde::{Deserialize, Serialize};

use super::defs::FeedViewPost;

/*
    app.bsky.feed.getAuthorFeed
*/

/// A response to getting a feed for an author.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetAuthorFeedResponse {
    /// A cursor for the stream.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// A list of posts.
    #[serde(rename = "feed")]
    pub feed: Vec<FeedViewPost>
}
