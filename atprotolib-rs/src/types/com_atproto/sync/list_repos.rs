use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.listRepos
*/

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

/// Represents a repository.
#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    /// The DID of the repository.
    #[serde(rename = "did")]
    pub did: String,

    /// The head of the repository.
    #[serde(rename = "head")]
    pub head: String,

    /// The latest revision of the repository.
    #[serde(rename = "rev")]
    pub rev: String,

    /// Whether the repository is active.
    #[serde(rename = "active", default)]
    pub active: bool,

    /// The status of the repository.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>
}
