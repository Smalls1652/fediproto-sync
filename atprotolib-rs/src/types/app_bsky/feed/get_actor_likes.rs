use serde::{Deserialize, Serialize};

use super::defs::FeedViewPost;

/*
    app.bsky.feed.getActorLikes
*/

/// A response to getting likes for an actor.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetActorLikesResponse {
    /// A cursor for the stream.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// A list of posts.
    #[serde(rename = "feed")]
    pub feed: Vec<FeedViewPost>
}
