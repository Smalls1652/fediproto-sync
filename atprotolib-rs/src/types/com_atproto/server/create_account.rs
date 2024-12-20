use serde::{Deserialize, Serialize};
/*
    com.atproto.server.createAccount
*/

/// Represents an account creation request.
///
/// [`com.atproto.server.createAccount#request`](https://docs.bsky.app/docs/api/com-atproto-server-create-account#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAccountRequest {
    /// Optional email address for the account.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Requested handle for the account.
    #[serde(rename = "handle")]
    pub handle: String,

    /// Pre-existing atproto DID, being imported to a new account.
    #[serde(rename = "did", skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,

    /// Optional invite code for the account.
    #[serde(rename = "inviteCode", skip_serializing_if = "Option::is_none")]
    pub invite_code: Option<String>,

    /// Optional phone number for the account.
    #[serde(rename = "verificationCode", skip_serializing_if = "Option::is_none")]
    pub verification_code: Option<String>,

    /// Optional phone number for the account.
    #[serde(rename = "verificationPhone", skip_serializing_if = "Option::is_none")]
    pub verification_phone: Option<String>,

    /// Initial account password. May need to meet instance-specific password
    /// strength requirements.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// DID PLC rotation key (aka, recovery key) to be included in PLC creation
    /// operation.
    #[serde(rename = "recoveryKey", skip_serializing_if = "Option::is_none")]
    pub recovery_key: Option<String>
}

/// Represents an account creation response.
///
/// [`com.atproto.server.createAccount#responses`](https://docs.bsky.app/docs/api/com-atproto-server-create-account#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAccountResponse {
    /// The access JWT for the new account.
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,

    /// The refresh JWT for the new account.
    #[serde(rename = "refreshJwt")]
    pub refresh_jwt: String,

    /// The handle of the new account.
    #[serde(rename = "handle")]
    pub handle: String,

    /// The DID of the new account.
    #[serde(rename = "did")]
    pub did: String
}
