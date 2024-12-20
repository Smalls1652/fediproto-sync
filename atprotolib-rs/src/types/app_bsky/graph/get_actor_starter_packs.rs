use serde::{Deserialize, Serialize};

use super::defs::StarterPackViewBasic;

/*
    app.bsky.graph.getActorStarterPacks
*/

/// The response to a request for a user's starter packs.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetActorStarterPacksResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's starter packs.
    starter_packs: Vec<StarterPackViewBasic>
}
