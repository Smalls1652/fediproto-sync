use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.disableAccountInvites
*/

/// Represents a request to disable account invites.
///
/// [`com.atproto.admin.disableAccountInvites#request`](https://docs.bsky.app/docs/api/com-atproto-admin-disable-account-invites#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct DisableAccountInvitesRequest {
    /// The DID of the account.
    #[serde(rename = "account")]
    pub account: String,

    /// Optional reason for disabled invites.
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>
}
