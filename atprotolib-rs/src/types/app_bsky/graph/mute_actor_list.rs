use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.muteActorList
*/

/// The request to mute an actor list.
#[derive(Serialize, Deserialize, Debug)]
pub struct MuteActorListRequest {
    /// The list.
    list: String
}
