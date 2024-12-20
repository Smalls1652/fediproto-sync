use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.getAccountInfo
*/

/// Represents a request to get account information.
///
/// [`com.atproto.admin.getAccountInfo#request`](https://docs.bsky.app/docs/api/com-atproto-admin-get-account-info#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountInfoRequest {
    /// The DID of the account to get information for.
    #[serde(rename = "did")]
    pub did: String
}

/// Represents a response to a request to get account information.
///
/// [`com.atproto.admin.getAccountInfo#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-account-info#responses)
#[allow(missing_docs)] // This should be replaced with just the response being `AccountView`.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountInfoResponse {
    #[serde(rename = "did")]
    pub did: String,

    #[serde(rename = "handle")]
    pub handle: String,

    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>,

    #[serde(rename = "invitesDisabled", default)]
    pub invites_disabled: bool,

    #[serde(rename = "emailConfirmedAt", skip_serializing_if = "Option::is_none")]
    pub email_confirmed_at: Option<DateTime<Utc>>,

    #[serde(rename = "inviteNote", skip_serializing_if = "Option::is_none")]
    pub invite_note: Option<String>,

    #[serde(rename = "deactivatedAt", skip_serializing_if = "Option::is_none")]
    pub deactivated_at: Option<DateTime<Utc>>,

    #[serde(rename = "threatSignatures", skip_serializing_if = "Option::is_none")]
    pub threat_signatures: Option<Vec<GetAccountInfoResponseThreatSignatures>>
}

#[allow(missing_docs)] // This should be replaced with just the response being `AccountView`.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountInfoResponseThreatSignatures {
    #[serde(rename = "property")]
    pub property: String,

    #[serde(rename = "value")]
    pub value: String
}
