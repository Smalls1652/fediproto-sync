use serde::{Deserialize, Serialize};

use super::defs::ListView;

/*
    app.bsky.graph.getListMutes
*/

/// The response to a request for a user's mute lists.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetListMutesResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's mute lists.
    lists: Vec<ListView>
}
