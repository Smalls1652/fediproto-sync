use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.subscribeRepos
*/

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    #[serde(rename = "seq", default)]
    pub seq: i32,
    #[serde(rename = "rebase", default)]
    pub rebase: bool,
    #[serde(rename = "tooBig", default)]
    pub too_big: bool,
    #[serde(rename = "repo")]
    pub repo: String,
    #[serde(rename = "commit")]
    pub commit: String,
    #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(rename = "rev")]
    pub rev: String,
    #[serde(rename = "since")]
    pub since: String,
    #[serde(rename = "blocks")]
    pub blocks: Vec<u8>,
    #[serde(rename = "ops")]
    pub ops: Vec<RepoOp>,
    #[serde(rename = "blobs")]
    pub blobs: Vec<String>,
    #[serde(rename = "time")]
    pub time: String
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
    #[serde(rename = "seq", default)]
    pub seq: i32,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "time")]
    pub time: DateTime<Utc>,
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    #[serde(rename = "seq", default)]
    pub seq: i32,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "time")]
    pub time: DateTime<Utc>,
    #[serde(rename = "active", default)]
    pub active: bool,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Handle {
    #[serde(rename = "seq", default)]
    pub seq: i32,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "handle")]
    pub handle: String,
    #[serde(rename = "time")]
    pub time: DateTime<Utc>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Migrate {
    #[serde(rename = "seq", default)]
    pub seq: i32,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "migrateTo")]
    pub migrate_to: String,
    #[serde(rename = "time")]
    pub time: DateTime<Utc>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Tombstone {
    #[serde(rename = "seq", default)]
    pub seq: i32,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "time")]
    pub time: DateTime<Utc>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RepoOp {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "cid")]
    pub cid: String
}
