use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.listBlobs
*/

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
