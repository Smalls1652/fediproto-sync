use serde::{Deserialize, Serialize};

use super::defs::ListView;

/*
    app.bsky.graph.getListBlocks
*/

/// The response to a request for a user's blocked lists.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetListBlocksResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's blocked lists.
    lists: Vec<ListView>
}
