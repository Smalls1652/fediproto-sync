use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.muteThread
*/

/// The request to mute a thread.
#[derive(Serialize, Deserialize, Debug)]
pub struct MuteThreadRequest {
    /// The URI of the thread to mute.
    root: String
}
