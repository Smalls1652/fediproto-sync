use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.starterpack
*/

/// A feed item for a starter pack.
#[derive(Serialize, Deserialize, Debug)]
pub struct StarterPackFeedItem {
    /// The URI of the feed item.
    uri: String
}
