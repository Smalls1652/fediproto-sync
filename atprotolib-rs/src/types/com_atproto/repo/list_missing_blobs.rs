use serde::{Deserialize, Serialize};

/*
    com.atproto.repo.listMissingBlobs
*/

/*    Type: response
    Id: com.atproto.repo.listMissingBlobs#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - blobs: #recordBlob[] (JsonProperty: blobs) [Required]
*/
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ListMissingBlobsResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "blobs")]
    pub blobs: Vec<RecordBlob>
}

/*    Type: recordBlob
    Id: com.atproto.repo.listMissingBlobs#recordBlob
    Kind: object

    Properties:
    - cid: string (JsonProperty: cid) [Required]
    - record_uri: string (JsonProperty: recordUri) [Required]
*/
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RecordBlob {
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "recordUri")]
    pub record_uri: String
}
