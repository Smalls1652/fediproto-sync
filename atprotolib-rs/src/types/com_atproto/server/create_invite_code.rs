use std::fmt;

use serde::{Deserialize, Serialize};

/*
    com.atproto.server.createInviteCode
*/

/// Represents an invite code request.
///
/// [`com.atproto.server.createInviteCode#request`](https://docs.bsky.app/docs/api/com-atproto-server-create-invite-code#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct InviteCodeRequest {
    /// The amount of uses the invite code has.
    #[serde(rename = "useCount", default)]
    pub use_count: i32,

    /// Generated for a specific account.
    #[serde(rename = "forAccount", skip_serializing_if = "Option::is_none")]
    pub for_account: Option<String>
}

/// Represents an invite code response.
///
/// [`com.atproto.server.createInviteCode#responses`](https://docs.bsky.app/docs/api/com-atproto-server-create-invite-code#responses)
#[derive(Serialize, Deserialize)]
pub struct InviteCodeResponse {
    /// The invite code.
    #[serde(rename = "code")]
    pub code: String
}

impl fmt::Display for InviteCodeResponse {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}
