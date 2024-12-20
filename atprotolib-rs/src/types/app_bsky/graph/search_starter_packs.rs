use serde::{Deserialize, Serialize};

use super::defs::StarterPackViewBasic;

/*
    app.bsky.graph.searchStarterPacks
*/

/// The response to a request to search for starter packs.
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchStarterPacksResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the starter packs.
    starter_packs: Vec<StarterPackViewBasic>
}
