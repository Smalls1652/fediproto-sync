use serde::{Deserialize, Serialize};

use super::CommitMeta;

/*
    com.atproto.repo.deleteRecord
*/

/*    Type: request
    Id: com.atproto.repo.deleteRecord#request
    Kind: object

    Properties:
    - repo: string (JsonProperty: repo) [Required]
    - collection: string (JsonProperty: collection) [Required]
    - rkey: string (JsonProperty: rkey) [Required]
    - swap_record: string (JsonProperty: swapRecord) [Optional]
    - swap_commit: string (JsonProperty: swapCommit) [Optional]
*/
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteRecordRequest {
    #[serde(rename = "repo")]
    pub repo: String,
    #[serde(rename = "collection")]
    pub collection: String,
    #[serde(rename = "rkey")]
    pub rkey: String,
    #[serde(rename = "swapRecord", skip_serializing_if = "Option::is_none")]
    pub swap_record: Option<String>,
    #[serde(rename = "swapCommit", skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<String>
}

/*    Type: response
    Id: com.atproto.repo.deleteRecord#response
    Kind: object

    Properties:
    - commit: com.atproto.repo.defs#commitMeta (JsonProperty: commit) [Optional]
*/
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteRecordResponse {
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<CommitMeta>
}
