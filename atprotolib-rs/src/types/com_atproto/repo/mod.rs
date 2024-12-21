#[cfg(feature = "apicalls")]
pub mod api_calls;

pub mod api_requests;
pub mod api_responses;

use serde::{Deserialize, Serialize};

use crate::types::app_bsky;

/// Metadata for a commit.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.repo.defs#commitMeta")]
pub struct CommitMeta {
    /// The CID.
    #[serde(rename = "cid")]
    pub cid: String,

    /// The revision.
    #[serde(rename = "rev")]
    pub rev: String
}

/// A URI with a content-hash fingerprint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StrongRef {
    /// The URI of the record.
    #[serde(rename = "uri")]
    pub uri: String,

    /// The CID of the record.
    #[serde(rename = "cid")]
    pub cid: String
}

/// Represents the different types of values that can be written.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type")]
pub enum WritesValue {
    /// A post.
    #[serde(rename = "app.bsky.feed.post")]
    Post(app_bsky::feed::Post)
}

/// Represents the type of write to apply.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type")]
pub enum RequestWrites {
    /// Create a new record.
    #[serde(rename = "com.atproto.repo.applyWrites#create")]
    Create(WriteCreate),

    /// Update an existing record.
    #[serde(rename = "com.atproto.repo.applyWrites#update")]
    Update(WriteUpdate),

    /// Delete an existing record.
    #[serde(rename = "com.atproto.repo.applyWrites#delete")]
    Delete(WriteDelete)
}

/// Represents a "create" write operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct WriteCreate {
    /// The collection to create the record in.
    #[serde(rename = "collection")]
    pub collection: String,

    /// The record key.
    #[serde(rename = "rkey", skip_serializing_if = "Option::is_none")]
    pub rkey: Option<String>,

    /// The value to create.
    #[serde(rename = "value")]
    pub value: WritesValue
}

impl WriteCreate {
    /// Creates a new `Create` struct.
    /// 
    /// ## Arguments
    /// 
    /// * `collection` - The collection to create the record in.
    /// * `value` - The value to create.
    pub fn new(
        collection: &str,
        value: WritesValue
    ) -> WriteCreate {
        WriteCreate {
            collection: collection.to_string(),
            rkey: None,
            value
        }
    }
}

/// Represents an "update" write operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct WriteUpdate {
    /// The collection to update the record in.
    #[serde(rename = "collection")]
    pub collection: String,

    /// The record key.
    #[serde(rename = "rkey")]
    pub rkey: String,

    /// The value to update.
    #[serde(rename = "value")]
    pub value: WritesValue
}

impl WriteUpdate {
    /// Creates a new `Update` struct.
    /// 
    /// ## Arguments
    /// 
    /// * `collection` - The collection to update the record in.
    /// * `rkey` - The record key.
    /// * `value` - The value to update.
    pub fn new(
        collection: &str,
        rkey: &str,
        value: WritesValue
    ) -> WriteUpdate {
        WriteUpdate {
            collection: collection.to_string(),
            rkey: rkey.to_string(),
            value
        }
    }
}

/// Represents a "delete" write operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct WriteDelete {
    /// The collection to delete the record from.
    #[serde(rename = "collection")]
    pub collection: String,

    /// The record key.
    #[serde(rename = "rkey")]
    pub rkey: String
}

impl WriteDelete {
    /// Creates a new `Delete` struct.
    /// 
    /// ## Arguments
    /// 
    /// * `collection` - The collection to delete the record from.
    /// * `rkey` - The record key.
    pub fn new(
        collection: &str,
        rkey: &str
    ) -> WriteDelete {
        WriteDelete {
            collection: collection.to_string(),
            rkey: rkey.to_string()
        }
    }
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RecordBlob {
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "recordUri")]
    pub record_uri: String
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "value")]
    pub value: serde_json::Value
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlobItem {
    #[serde(rename = "$type")]
    pub item_type: String,

    #[serde(rename = "mimeType")]
    pub mime_type: String,

    #[serde(rename = "ref")]
    pub item_ref: BlobItemRef,

    #[serde(rename = "size")]
    pub size: u64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlobItemRef {
    #[serde(rename = "$link")]
    pub link: String
}
