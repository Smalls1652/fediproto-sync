use serde::{Deserialize, Serialize};

use super::{BlobItem, CommitMeta, Record, RecordBlob};

/// Represents a response to a request to apply writes.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplyWritesResponse {
    /// The commit metadata.
    #[serde(rename = "commit")]
    pub commit: CommitMeta,

    /// The results of the writes.
    #[serde(rename = "results")]
    pub results: Vec<ApplyWritesResponseResults>
}

/// Represents the results of the writes.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type")]
pub enum ApplyWritesResponseResults {
    /// The result of a create operation.
    #[serde(rename = "com.atproto.repo.applyWrites#createResult")]
    CreateResult(WriteCreateResult),

    /// The result of an update operation.
    #[serde(rename = "com.atproto.repo.applyWrites#updateResult")]
    UpdateResult(WriteUpdateResult),

    /// The result of a delete operation.
    #[serde(rename = "com.atproto.repo.applyWrites#deleteResult")]
    DeleteResult(WriteDeleteResult)
}

/// Represents the results of a "create" write operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct WriteCreateResult {
    /// The URI of the created record.
    #[serde(rename = "uri")]
    pub uri: String,

    /// The CID of the created record.
    #[serde(rename = "cid")]
    pub cid: String,

    /// The validation status of the created record.
    #[serde(rename = "validationStatus", skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>
}

/// Represents the results of an "update" write operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct WriteUpdateResult {
    /// The URI of the updated record.
    #[serde(rename = "uri")]
    pub uri: String,

    /// The CID of the updated record.
    #[serde(rename = "cid")]
    pub cid: String,

    /// The validation status of the updated record.
    #[serde(rename = "validationStatus", skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>
}

/// Represents the results of a "delete" write operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct WriteDeleteResult {}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRecordResponse {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<CommitMeta>,
    #[serde(rename = "validationStatus", skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteRecordResponse {
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<CommitMeta>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DescribeRepoResponse {
    #[serde(rename = "handle")]
    pub handle: String,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "didDoc")]
    pub did_doc: serde_json::Value,
    #[serde(rename = "collections")]
    pub collections: Vec<String>,
    #[serde(rename = "handleIsCorrect", default)]
    pub handle_is_correct: bool
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RecordResponse {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "value")]
    pub value: serde_json::Value
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ListMissingBlobsResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "blobs")]
    pub blobs: Vec<RecordBlob>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ListRecordsResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "records")]
    pub records: Vec<Record>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PutRecordResponse {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<CommitMeta>,
    #[serde(rename = "validationStatus", skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct UploadBlobResponse {
    #[serde(rename = "blob")]
    pub blob: BlobItem
}
