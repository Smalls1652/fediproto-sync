use serde::{Deserialize, Serialize};

use super::defs::ListView;

/*
    app.bsky.graph.getLists
*/

/// The response to a request for lists.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetListsResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the lists.
    lists: Vec<ListView>
}
