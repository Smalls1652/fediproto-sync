use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.deleteAccount
*/

/// Represents an account deletion request.
///
/// [`com.atproto.admin.deleteAccount#request`](https://docs.bsky.app/docs/api/com-atproto-admin-delete-account#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAccountRequest {
    /// The DID of the account to delete.
    #[serde(rename = "did")]
    pub did: String
}
