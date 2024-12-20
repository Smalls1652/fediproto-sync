use serde::{Deserialize, Serialize};

/*
    com.atproto.identity.resolveHandle
*/

/// Represents a response to a request to resolve a handle to a DID.
#[derive(Serialize, Deserialize, Debug)]
pub struct ResolveHandleResponse {
    /// The DID associated with the handle.
    #[serde(rename = "did")]
    pub did: String
}
