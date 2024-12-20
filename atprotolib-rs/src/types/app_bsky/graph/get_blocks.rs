use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

/*
    app.bsky.graph.getBlocks
*/

/// The response to a request for a user's blocked profiles.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlocksResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's blocked profiles.
    blocks: Vec<ProfileView>
}
