use serde::{Deserialize, Serialize};

use super::defs::StarterPackView;

/*
    app.bsky.graph.getStarterPack
*/

/// The response to a request for a starter pack.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetStarterPackResponse {
    /// The starter pack.
    starter_pack: StarterPackView
}
