use serde::{Deserialize, Serialize};

use super::defs::StarterPackViewBasic;

/*
    app.bsky.graph.getStarterPacks
*/

/// The response to a request for starter packs.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetStarterPacksResponse {
    /// A list of the starter packs.
    starter_packs: Vec<StarterPackViewBasic>
}
