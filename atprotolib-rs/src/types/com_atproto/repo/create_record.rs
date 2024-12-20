use serde::{Deserialize, Serialize};

use super::CommitMeta;

/*
    com.atproto.repo.createRecord
*/

/*    Type: request
    Id: com.atproto.repo.createRecord#request
    Kind: object

    Properties:
    - repo: string (JsonProperty: repo) [Required]
    - collection: string (JsonProperty: collection) [Required]
    - rkey: string (JsonProperty: rkey) [Optional]
    - validate: boolean  (JsonProperty: validate) [Optional]
    - record: unknown  (JsonProperty: record) [Required]
    - swap_commit: string (JsonProperty: swapCommit) [Optional]
*/
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRecordRequest {
    #[serde(rename = "repo")]
    pub repo: String,
    #[serde(rename = "collection")]
    pub collection: String,
    #[serde(rename = "rkey", skip_serializing_if = "Option::is_none")]
    pub rkey: Option<String>,
    #[serde(rename = "validate", default)]
    pub validate: bool,
    #[serde(rename = "record")]
    pub record: serde_json::Value,
    #[serde(rename = "swapCommit", skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<String>
}

/*    Type: response
    Id: com.atproto.repo.createRecord#response
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - commit: com.atproto.repo.defs#commitMeta (JsonProperty: commit) [Optional]
    - validation_status: string (JsonProperty: validationStatus) [Optional]
*/
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRecordResponse {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<CommitMeta>,
    #[serde(rename = "validationStatus", skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>
}
