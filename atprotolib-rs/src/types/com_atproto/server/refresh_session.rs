use serde::{Deserialize, Serialize};

use super::DidDoc;

/*
    com.atproto.server.refreshSession
*/

/// Represents a session refresh response.
///
/// [`com.atproto.server.refreshSession#responses`](https://docs.bsky.app/docs/api/com-atproto-server-refresh-session#responses)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefreshSessionResponse {
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

    /// Whether the session is active.
    #[serde(rename = "active", default)]
    pub active: bool,

    /// The status of the session.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>
}
