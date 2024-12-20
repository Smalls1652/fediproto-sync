use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.updateAccountPassword
*/

/// Represents a request to update an account's password.
///
/// [`com.atproto.admin.updateAccountPassword#request`](https://docs.bsky.app/docs/api/com-atproto-admin-update-account-password#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountPasswordRequest {
    /// The DID of the account.
    #[serde(rename = "did")]
    pub did: String,

    /// The new password.
    #[serde(rename = "password")]
    pub password: String
}
