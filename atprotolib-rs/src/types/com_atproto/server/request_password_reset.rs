use serde::{Deserialize, Serialize};

/*
    com.atproto.server.requestPasswordReset
*/

/// Represents a password reset request.
///
/// [`com.atproto.server.requestPasswordReset#request`](https://docs.bsky.app/docs/api/com-atproto-server-request-password-reset#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPasswordResetRequest {
    /// The email of the account to reset the password for.
    #[serde(rename = "email")]
    pub email: String
}
