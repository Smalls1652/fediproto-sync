use serde::{Deserialize, Serialize};

/*
    com.atproto.identity.updateHandle
*/

/// Represents a request to update a handle.
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHandleRequest {
    /// The new handle.
    #[serde(rename = "handle")]
    pub handle: String
}
