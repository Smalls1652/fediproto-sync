use serde::{Deserialize, Serialize};

/*
    com.atproto.server.createInviteCodes
*/

/*    Type: accountCodes
    Id: com.atproto.server.createInviteCodes#accountCodes
    Kind: object

    Properties:
    - account: string (JsonProperty: account) [Required]
    - codes: string[] (JsonProperty: codes) [Required]
*/

/// Represents invite codes generated for an account.
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountCodes {
    /// The account DID.
    #[serde(rename = "account")]
    pub account: String,

    /// The invite codes generated.
    #[serde(rename = "codes")]
    pub codes: Vec<String>
}
