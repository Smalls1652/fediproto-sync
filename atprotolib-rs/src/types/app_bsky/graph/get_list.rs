use serde::{Deserialize, Serialize};

use super::defs::{ListItemView, ListView};

/*
    app.bsky.graph.getList
*/

/// The response to a request for a list.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetListResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// The list.
    list: ListView,

    /// A list of the list's items.
    items: Vec<ListItemView>
}
