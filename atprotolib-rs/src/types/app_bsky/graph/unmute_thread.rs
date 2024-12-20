use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.unmuteThread
*/

/// The request to unmute a thread.
#[derive(Serialize, Deserialize, Debug)]
pub struct UnmuteThreadRequest {
    /// The URI of the thread to unmute.
    root: String
}
