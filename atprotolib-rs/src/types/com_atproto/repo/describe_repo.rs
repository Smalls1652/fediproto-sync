use serde::{Deserialize, Serialize};

/*
    com.atproto.repo.describeRepo
*/

/*    Type: response
    Id: com.atproto.repo.describeRepo#response
    Kind: object

    Properties:
    - handle: string (JsonProperty: handle) [Required]
    - did: string (JsonProperty: did) [Required]
    - did_doc: unknown  (JsonProperty: didDoc) [Required]
    - collections: string[] (JsonProperty: collections) [Required]
    - handle_is_correct: boolean  (JsonProperty: handleIsCorrect) [Required]
*/
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DescribeRepoResponse {
    #[serde(rename = "handle")]
    pub handle: String,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "didDoc")]
    pub did_doc: serde_json::Value,
    #[serde(rename = "collections")]
    pub collections: Vec<String>,
    #[serde(rename = "handleIsCorrect", default)]
    pub handle_is_correct: bool
}
