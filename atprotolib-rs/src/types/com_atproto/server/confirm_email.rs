use serde::{Deserialize, Serialize};

/*
    com.atproto.server.confirmEmail
*/

/// Represents a confirm email request.
///
/// [`com.atproto.server.confirmEmail#request`](https://docs.bsky.app/docs/api/com-atproto-server-confirm-email#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfirmEmailRequest {
    /// The email to confirm.
    #[serde(rename = "email")]
    pub email: String,

    /// The confirmation token.
    #[serde(rename = "token")]
    pub token: String
}
