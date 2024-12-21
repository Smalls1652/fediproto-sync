use serde::{Deserialize, Serialize};

use super::Repo;

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct HeadResponse {
    #[serde(rename = "root")]
    pub root: String
}

/// Represents a response to a request to get the latest commit.
#[derive(Serialize, Deserialize, Debug)]
pub struct LatestCommitResponse {
    /// The CID of the latest commit.
    #[serde(rename = "cid")]
    pub cid: String,

    /// The revision of the latest commit.
    #[serde(rename = "rev")]
    pub rev: String
}

/// Represents a response to a request to get the status of a repository.
#[derive(Serialize, Deserialize, Debug)]
pub struct RepoStatusResponse {
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

/// Represents a response to a request to list blobs.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListBlobsResponse {
    /// The cursor stream position.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// The CIDs of the blobs.
    #[serde(rename = "cids")]
    pub cids: Vec<String>
}

/// Represents a response to a request to list repos.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListReposResponse {
    /// The cursor stream position.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// The repositories.
    #[serde(rename = "repos")]
    pub repos: Vec<Repo>
}
