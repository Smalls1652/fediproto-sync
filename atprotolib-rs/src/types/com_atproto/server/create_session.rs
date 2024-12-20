use serde::{Deserialize, Serialize};

use super::DidDoc;

/*
    com.atproto.server.createSession
*/

/// Represents a session creation request.
///
/// [`com.atproto.server.createSession#request`](https://docs.bsky.app/docs/api/com-atproto-server-create-session#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSessionRequest {
    /// Handle or other identifier supported by the server for the
    /// authenticating user.
    #[serde(rename = "identifier")]
    pub identifier: String,

    /// The password of the authenticating user.
    #[serde(rename = "password")]
    pub password: String,

    /// An optional token for two-factor authentication.
    #[serde(rename = "authFactorToken", skip_serializing_if = "Option::is_none")]
    pub auth_factor_token: Option<String>
}

/// Represents a session creation response.
///
/// [`com.atproto.server.createSession#responses`](https://docs.bsky.app/docs/api/com-atproto-server-create-session#responses)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSessionResponse {
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

    /// DID document of the account.
    #[serde(rename = "didDoc", skip_serializing_if = "Option::is_none")]
    pub did_doc: Option<DidDoc>,

    /// The email of the account.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

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
