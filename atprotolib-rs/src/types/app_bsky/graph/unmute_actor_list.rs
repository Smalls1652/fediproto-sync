use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.unmuteActorList
*/

/// The request to unmute an actor list.
#[derive(Serialize, Deserialize, Debug)]
pub struct UnmuteActorListRequest {
    /// The list.
    list: String
}
