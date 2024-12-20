use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.updateAccountEmail
*/

/// Represents a request to update an account's email.
///
/// [`com.atproto.admin.updateAccountEmail#request`](https://docs.bsky.app/docs/api/com-atproto-admin-update-account-email#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountEmailRequest {
    /// The DID of the account.
    #[serde(rename = "account")]
    pub account: String,

    /// The new email address.
    #[serde(rename = "email")]
    pub email: String
}
