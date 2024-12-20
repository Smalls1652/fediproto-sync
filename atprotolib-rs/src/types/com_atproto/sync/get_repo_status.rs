use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.getRepoStatus
*/

/// Represents a response to a request to get the status of a repository.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetRepoStatusResponse {
    /// The DID of the repository.
    #[serde(rename = "did")]
    pub did: String,

    /// Whether the repository is active.
    #[serde(rename = "active", default)]
    pub active: bool,

    /// The status of the repository.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The latest revision of the repository.
    #[serde(rename = "rev", skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>
}
