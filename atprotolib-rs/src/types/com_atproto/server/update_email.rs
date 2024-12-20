use serde::{Deserialize, Serialize};

/*
    com.atproto.server.updateEmail
*/

/// Represents an email update request.
///
/// [`com.atproto.server.updateEmail#request`](https://docs.bsky.app/docs/api/com-atproto-server-update-email#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateEmailRequest {
    /// The email address.
    #[serde(rename = "email")]
    pub email: String,

    /// Whether to require email authentication.
    #[serde(rename = "emailAuthFactor", default)]
    pub email_auth_factor: bool,

    /// Requires a token from `com.atproto.sever.requestEmailUpdate` if the
    /// account's email has been confirmed.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>
}
