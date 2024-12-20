use serde::{Deserialize, Serialize};

/*
    com.atproto.server.revokeAppPassword
*/

/// Represents an app password revocation request.
///
/// [`com.atproto.server.revokeAppPassword#request`](https://docs.bsky.app/docs/api/com-atproto-server-revoke-app-password#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct RevokeAppPasswordRequest {
    /// The name of the app password to revoke.
    #[serde(rename = "name")]
    pub name: String
}
