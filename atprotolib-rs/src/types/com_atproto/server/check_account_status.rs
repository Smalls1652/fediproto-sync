use serde::{Deserialize, Serialize};

/*
    com.atproto.server.checkAccountStatus
*/

/// Represents an account status response.
///
/// [`com.atproto.server.checkAccountStatus#responses`](https://docs.bsky.app/docs/api/com-atproto-server-check-account-status#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct CheckAccountStatusResponse {
    /// Whether the account is activated.
    #[serde(rename = "activated", default)]
    pub activated: bool,

    /// Whether the account's DID is valid.
    #[serde(rename = "validDid", default)]
    pub valid_did: bool,

    /// The repo commit CID.
    #[serde(rename = "repoCommit")]
    pub repo_commit: String,

    /// The repo revision.
    #[serde(rename = "repoRev")]
    pub repo_revision: String,

    /// The repo blocks.
    #[serde(rename = "repoBlocks")]
    pub repo_blocks: String,

    /// The count of indexed records.
    #[serde(rename = "indexedRecords", default)]
    pub indexed_records: i32,

    /// The count of private state values.
    #[serde(rename = "privateStateValues", default)]
    pub private_state_values: i32,

    /// The count of expected blobs.
    #[serde(rename = "publicStateValues", default)]
    pub expected_blobs: i32,

    /// The count of imported blobs.
    #[serde(rename = "importedBlobs", default)]
    pub imported_blobs: i32
}
