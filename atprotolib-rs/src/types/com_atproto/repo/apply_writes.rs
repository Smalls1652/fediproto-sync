use serde::{Deserialize, Serialize};

use super::CommitMeta;
use crate::types::app_bsky;

/*
    com.atproto.repo.applyWrites
*/

/// Represents a request to apply writes.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplyWritesRequest {
    /// The handle or DID of the repo (aka, current account).
    #[serde(rename = "repo")]
    pub repo: String,

    /// Can be set to 'false' to skip Lexicon schema validation of record data across all operations, 'true' to require it, or leave unset to validate only for known Lexicons.
    #[serde(rename = "validate", default)]
    pub validate: bool,

    /// The writes to apply.
    #[serde(rename = "writes")]
    pub writes: Vec<ApplyWritesRequestWrites>,

    /// If provided, the entire operation will fail if the current repo commit CID does not match this value. Used to prevent conflicting repo mutations.
    #[serde(rename = "swapCommit", skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<String>
}

/// Represents the type of write to apply.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type")]
pub enum ApplyWritesRequestWrites {
    /// Create a new record.
    #[serde(rename = "com.atproto.repo.applyWrites#create")]
    Create(Create),

    /// Update an existing record.
    #[serde(rename = "com.atproto.repo.applyWrites#update")]
    Update(Update),

    /// Delete an existing record.
    #[serde(rename = "com.atproto.repo.applyWrites#delete")]
    Delete(Delete)
}

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
    CreateResult(CreateResult),

    /// The result of an update operation.
    #[serde(rename = "com.atproto.repo.applyWrites#updateResult")]
    UpdateResult(UpdateResult),

    /// The result of a delete operation.
    #[serde(rename = "com.atproto.repo.applyWrites#deleteResult")]
    DeleteResult(DeleteResult)
}

/// Represents the different types of values that can be written.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type")]
pub enum ApplyWritesValue {
    /// A post.
    #[serde(rename = "app.bsky.feed.post")]
    Post(app_bsky::feed::Post)
}

/// Represents a "create" write operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct Create {
    /// The collection to create the record in.
    #[serde(rename = "collection")]
    pub collection: String,

    /// The record key.
    #[serde(rename = "rkey", skip_serializing_if = "Option::is_none")]
    pub rkey: Option<String>,

    /// The value to create.
    #[serde(rename = "value")]
    pub value: ApplyWritesValue
}

impl Create {
    /// Creates a new `Create` struct.
    /// 
    /// ## Arguments
    /// 
    /// * `collection` - The collection to create the record in.
    /// * `value` - The value to create.
    pub fn new(
        collection: &str,
        value: ApplyWritesValue
    ) -> Create {
        Create {
            collection: collection.to_string(),
            rkey: None,
            value
        }
    }
}

/// Represents an "update" write operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    /// The collection to update the record in.
    #[serde(rename = "collection")]
    pub collection: String,

    /// The record key.
    #[serde(rename = "rkey")]
    pub rkey: String,

    /// The value to update.
    #[serde(rename = "value")]
    pub value: ApplyWritesValue
}

impl Update {
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
        value: ApplyWritesValue
    ) -> Update {
        Update {
            collection: collection.to_string(),
            rkey: rkey.to_string(),
            value
        }
    }
}

/// Represents a "delete" write operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct Delete {
    /// The collection to delete the record from.
    #[serde(rename = "collection")]
    pub collection: String,

    /// The record key.
    #[serde(rename = "rkey")]
    pub rkey: String
}

impl Delete {
    /// Creates a new `Delete` struct.
    /// 
    /// ## Arguments
    /// 
    /// * `collection` - The collection to delete the record from.
    /// * `rkey` - The record key.
    pub fn new(
        collection: &str,
        rkey: &str
    ) -> Delete {
        Delete {
            collection: collection.to_string(),
            rkey: rkey.to_string()
        }
    }
}

/// Represents the results of a "create" write operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateResult {
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
pub struct UpdateResult {
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
pub struct DeleteResult {}
