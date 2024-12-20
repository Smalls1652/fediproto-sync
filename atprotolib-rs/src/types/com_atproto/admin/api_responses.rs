use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a response to a request to get account information.
///
/// [`com.atproto.admin.getAccountInfo#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-account-info#responses)
#[allow(missing_docs)] // This should be replaced with just the response being `AccountView`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfoResponse {
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
    pub threat_signatures: Option<Vec<AccountInfoResponseThreatSignatures>>
}

#[allow(missing_docs)] // This should be replaced with just the response being `AccountView`.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfoResponseThreatSignatures {
    #[serde(rename = "property")]
    pub property: String,

    #[serde(rename = "value")]
    pub value: String
}

/// Represents a response to a request to get account information for multiple
/// accounts.
///
/// [`com.atproto.admin.getAccountInfos#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-account-infos#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfosResponse {
    /// Account information for the requested accounts.
    #[serde(rename = "infos")]
    pub infos: Vec<AccountInfoResponse>
}

/// Represents a response to get invite codes.
///
/// [`com.atproto.admin.getInviteCodes#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-invite-codes#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct InviteCodesResponse {
    /// The cursor stream position.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// The invite codes.
    #[serde(rename = "codes")]
    pub codes: Vec<InviteCodesResponseCode>
}

#[allow(missing_docs)] // This should be replaced with just the response being `InviteCode`.
#[derive(Debug, Serialize, Deserialize)]
pub struct InviteCodesResponseCode {
    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "available", default)]
    pub available: i32,

    #[serde(rename = "disabled", default)]
    pub disabled: bool,

    #[serde(rename = "forAccount")]
    pub for_account: String,

    #[serde(rename = "createdBy")]
    pub created_by: String,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "uses")]
    pub uses: Vec<InviteCodesResponseCodeUse>
}

#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct InviteCodesResponseCodeUse {
    #[serde(rename = "usedBy")]
    pub used_by: String,

    #[serde(rename = "usedAt")]
    pub used_at: DateTime<Utc>
}

/// Represents a response to get the status of an Account subject.
///
/// [`com.atproto.admin.getSubjectStatus#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-subject-status#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectStatusAccountResponse {
    /// The DID.
    #[serde(rename = "did")]
    pub did: String,

    /// The status of the takedown request.
    #[serde(rename = "takedown")]
    pub takedown: SubjectStatusResponseStatus,

    /// The status of the deactivation request.
    #[serde(rename = "deactivated")]
    pub deactivated: SubjectStatusResponseStatus
}

/// Represents a response to get the status of a Record subject.
///
/// [`com.atproto.admin.getSubjectStatus#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-subject-status#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectStatusRecordResponse {
    /// The URI of the record.
    #[serde(rename = "uri")]
    pub uri: String,

    /// The CID of the record.
    #[serde(rename = "cid")]
    pub cid: String,

    /// The status of the takedown request.
    #[serde(rename = "takedown")]
    pub takedown: SubjectStatusResponseStatus,

    /// The status of the deactivation request.
    #[serde(rename = "deactivated")]
    pub deactivated: SubjectStatusResponseStatus
}

/// Represents a response to get the status of a Blob subject.
///
/// [`com.atproto.admin.getSubjectStatus#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-subject-status#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectStatusBlobResponse {
    /// The DID of the blob.
    #[serde(rename = "did")]
    pub did: String,

    /// The CID of the blob.
    #[serde(rename = "cid")]
    pub cid: String,

    /// The URI of the record.
    #[serde(rename = "recordUri", skip_serializing_if = "Option::is_none")]
    pub record_uri: Option<String>,

    /// The status of the takedown request.
    #[serde(rename = "takedown")]
    pub takedown: SubjectStatusResponseStatus,

    /// The status of the deactivation request.
    #[serde(rename = "deactivated")]
    pub deactivated: SubjectStatusResponseStatus
}

/// Represents the status for a takedown or deactivation request.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectStatusResponseStatus {
    /// Whether the request has been applied.
    #[serde(rename = "applied", default)]
    pub applied: bool,

    /// The reference of the request.
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>
}

/// Represents a response to a search for accounts.
///
/// [`com.atproto.admin.searchAccounts#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-search-accounts#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAccountsResponse {
    /// The cursor stream position.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// The accounts that match the search criteria.
    #[serde(rename = "accounts")]
    pub accounts: Vec<AccountInfoResponse>
}

/// Represents a response to a request to send an email.
///
/// [`com.atproto.admin.sendEmail#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-send-email#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct SendEmailResponse {
    /// Whether the email was sent.
    #[serde(rename = "sent", default)]
    pub sent: bool
}
