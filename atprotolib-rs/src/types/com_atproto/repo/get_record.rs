use serde::{Deserialize, Serialize};

/*
    com.atproto.repo.getRecord
*/

/*    Type: response
    Id: com.atproto.repo.getRecord#response
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Optional]
    - value: unknown  (JsonProperty: value) [Required]
*/
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetRecordResponse {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "value")]
    pub value: serde_json::Value
}
