use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.disableInviteCodes
*/

/// Represents a request to disable invite codes.
///
/// [`com.atproto.admin.disableInviteCodes#request`](https://docs.bsky.app/docs/api/com-atproto-admin-disable-invite-codes#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct DisableInviteCodesRequest {
    /// The invite codes to disable.
    #[serde(rename = "codes", skip_serializing_if = "Option::is_none")]
    pub codes: Option<Vec<String>>,

    /// The accounts to disable invite codes for.
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>
}
