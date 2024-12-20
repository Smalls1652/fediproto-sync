use serde::{Deserialize, Serialize};

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
