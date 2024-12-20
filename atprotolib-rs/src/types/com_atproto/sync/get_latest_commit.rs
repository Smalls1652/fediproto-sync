use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.getLatestCommit
*/

/// Represents a response to a request to get the latest commit.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetLatestCommitResponse {
    /// The CID of the latest commit.
    #[serde(rename = "cid")]
    pub cid: String,

    /// The revision of the latest commit.
    #[serde(rename = "rev")]
    pub rev: String
}
