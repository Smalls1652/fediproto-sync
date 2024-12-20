use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.getSubjectStatus
*/

/// Represents a response to get the status of an Account subject.
///
/// [`com.atproto.admin.getSubjectStatus#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-subject-status#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubjectStatusAccountResponse {
    /// The DID.
    #[serde(rename = "did")]
    pub did: String,

    /// The status of the takedown request.
    #[serde(rename = "takedown")]
    pub takedown: GetSubjectStatusResponseStatus,

    /// The status of the deactivation request.
    #[serde(rename = "deactivated")]
    pub deactivated: GetSubjectStatusResponseStatus
}

/// Represents a response to get the status of a Record subject.
///
/// [`com.atproto.admin.getSubjectStatus#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-subject-status#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubjectStatusRecordResponse {
    /// The URI of the record.
    #[serde(rename = "uri")]
    pub uri: String,

    /// The CID of the record.
    #[serde(rename = "cid")]
    pub cid: String,

    /// The status of the takedown request.
    #[serde(rename = "takedown")]
    pub takedown: GetSubjectStatusResponseStatus,

    /// The status of the deactivation request.
    #[serde(rename = "deactivated")]
    pub deactivated: GetSubjectStatusResponseStatus
}

/// Represents a response to get the status of a Blob subject.
///
/// [`com.atproto.admin.getSubjectStatus#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-subject-status#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubjectStatusBlobResponse {
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
    pub takedown: GetSubjectStatusResponseStatus,

    /// The status of the deactivation request.
    #[serde(rename = "deactivated")]
    pub deactivated: GetSubjectStatusResponseStatus
}

/// Represents the status for a takedown or deactivation request.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubjectStatusResponseStatus {
    /// Whether the request has been applied.
    #[serde(rename = "applied", default)]
    pub applied: bool,

    /// The reference of the request.
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>
}
