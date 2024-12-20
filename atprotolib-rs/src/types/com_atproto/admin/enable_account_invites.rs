use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.enableAccountInvites
*/

/// Represents a request to enable account invites.
///
/// [`com.atproto.admin.enableAccountInvites#request`](https://docs.bsky.app/docs/api/com-atproto-admin-enable-account-invites#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct EnableAccountInvitesRequest {
    /// The DID of the account.
    #[serde(rename = "account")]
    pub account: String,

    /// Optional reason for disabled invites.
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>
}
