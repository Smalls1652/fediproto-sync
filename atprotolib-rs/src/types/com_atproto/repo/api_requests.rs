use serde::{Deserialize, Serialize};

use super::RequestWrites;

/// Represents a request to apply writes.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplyWritesRequest {
    /// The handle or DID of the repo (aka, current account).
    #[serde(rename = "repo")]
    pub repo: String,

    /// Can be set to 'false' to skip Lexicon schema validation of record data
    /// across all operations, 'true' to require it, or leave unset to validate
    /// only for known Lexicons.
    #[serde(rename = "validate", default)]
    pub validate: bool,

    /// The writes to apply.
    #[serde(rename = "writes")]
    pub writes: Vec<RequestWrites>,

    /// If provided, the entire operation will fail if the current repo commit
    /// CID does not match this value. Used to prevent conflicting repo
    /// mutations.
    #[serde(rename = "swapCommit", skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<String>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRecordRequest {
    #[serde(rename = "repo")]
    pub repo: String,
    #[serde(rename = "collection")]
    pub collection: String,
    #[serde(rename = "rkey", skip_serializing_if = "Option::is_none")]
    pub rkey: Option<String>,
    #[serde(rename = "validate", default)]
    pub validate: bool,
    #[serde(rename = "record")]
    pub record: serde_json::Value,
    #[serde(rename = "swapCommit", skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<String>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteRecordRequest {
    #[serde(rename = "repo")]
    pub repo: String,
    #[serde(rename = "collection")]
    pub collection: String,
    #[serde(rename = "rkey")]
    pub rkey: String,
    #[serde(rename = "swapRecord", skip_serializing_if = "Option::is_none")]
    pub swap_record: Option<String>,
    #[serde(rename = "swapCommit", skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<String>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PutRecordRequest {
    #[serde(rename = "repo")]
    pub repo: String,
    #[serde(rename = "collection")]
    pub collection: String,
    #[serde(rename = "rkey")]
    pub rkey: String,
    #[serde(rename = "validate", default)]
    pub validate: bool,
    #[serde(rename = "record")]
    pub record: serde_json::Value,
    #[serde(rename = "swapRecord", skip_serializing_if = "Option::is_none")]
    pub swap_record: Option<String>,
    #[serde(rename = "swapCommit", skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<String>
}
