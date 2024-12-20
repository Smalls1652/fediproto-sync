use serde::{Deserialize, Serialize};

use super::defs::GeneratorView;

/*
    app.bsky.feed.getActorFeeds
*/

/// A response to getting feeds for an actor.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetActorFeedsResponse {
    /// A cursor for the stream.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// A list of feeds.
    #[serde(rename = "feeds")]
    pub feeds: Vec<GeneratorView>
}
