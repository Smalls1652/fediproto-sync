use serde::{Deserialize, Serialize};

/*
    com.atproto.repo.listRecords
*/

/*    Type: response
    Id: com.atproto.repo.listRecords#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - records: #record[] (JsonProperty: records) [Required]
*/
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ListRecordsResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "records")]
    pub records: Vec<Record>
}

/*    Type: record
    Id: com.atproto.repo.listRecords#record
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - value: unknown  (JsonProperty: value) [Required]
*/
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "value")]
    pub value: serde_json::Value
}
