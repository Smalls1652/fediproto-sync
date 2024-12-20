use serde::{Deserialize, Serialize};

/*
    com.atproto.repo.uploadBlob
*/

/*    Type: response
    Id: com.atproto.repo.uploadBlob#response
    Kind: object

    Properties:
    - blob: blob  (JsonProperty: blob) [Required]
*/
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct UploadBlobResponse {
    #[serde(rename = "blob")]
    pub blob: BlobItem
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlobItem {
    #[serde(rename = "$type")]
    pub item_type: String,

    #[serde(rename = "mimeType")]
    pub mime_type: String,

    #[serde(rename = "ref")]
    pub item_ref: BlobItemRef,

    #[serde(rename = "size")]
    pub size: u64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlobItemRef {
    #[serde(rename = "$link")]
    pub link: String
}
