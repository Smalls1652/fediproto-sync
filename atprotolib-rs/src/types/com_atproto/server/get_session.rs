use serde::{Deserialize, Serialize};

/*
    com.atproto.server.getSession
*/

/// Represents a session request.
///
/// [`com.atproto.server.getSession#request`](https://docs.bsky.app/docs/api/com-atproto-server-get-session#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct GetSessionResponse {
    /// The access JWT.
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,

    /// The refresh JWT.
    #[serde(rename = "refreshJwt")]
    pub refresh_jwt: String,

    /// The handle of the account.
    #[serde(rename = "handle")]
    pub handle: String,

    /// The DID of the account.
    #[serde(rename = "did")]
    pub did: String,

    /// The email of the account.
    #[serde(rename = "email")]
    pub email: String,

    /// Whether the email is confirmed.
    #[serde(rename = "emailConfirmed", default)]
    pub email_confirmed: bool,

    /// Whether email is used as an authentication factor.
    #[serde(rename = "emailAuthFactor", default)]
    pub email_auth_factor: bool,

    /// Whether the session is active.
    #[serde(rename = "active", default)]
    pub active: bool,

    /// The status of the session.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>
}
