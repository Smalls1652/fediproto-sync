use serde::{Deserialize, Serialize};

/*
    com.atproto.server.deleteAccount
*/

/// Represents an account deletion request.
///
/// [`com.atproto.server.deleteAccount#request`](https://docs.bsky.app/docs/api/com-atproto-server-delete-account#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteAccountRequest {
    /// The account's DID.
    #[serde(rename = "did")]
    pub did: String,

    /// The account's password.
    #[serde(rename = "password")]
    pub password: String,

    /// Confirmation token for the account deletion.
    #[serde(rename = "token")]
    pub token: String
}
