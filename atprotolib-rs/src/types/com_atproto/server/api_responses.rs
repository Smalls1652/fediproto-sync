use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{AppPassword, DidDoc, ServerContact, ServerLinks};

/// Represents an account status response.
///
/// [`com.atproto.server.checkAccountStatus#responses`](https://docs.bsky.app/docs/api/com-atproto-server-check-account-status#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct CheckAccountStatusResponse {
    /// Whether the account is activated.
    #[serde(rename = "activated", default)]
    pub activated: bool,

    /// Whether the account's DID is valid.
    #[serde(rename = "validDid", default)]
    pub valid_did: bool,

    /// The repo commit CID.
    #[serde(rename = "repoCommit")]
    pub repo_commit: String,

    /// The repo revision.
    #[serde(rename = "repoRev")]
    pub repo_revision: String,

    /// The repo blocks.
    #[serde(rename = "repoBlocks")]
    pub repo_blocks: String,

    /// The count of indexed records.
    #[serde(rename = "indexedRecords", default)]
    pub indexed_records: i32,

    /// The count of private state values.
    #[serde(rename = "privateStateValues", default)]
    pub private_state_values: i32,

    /// The count of expected blobs.
    #[serde(rename = "publicStateValues", default)]
    pub expected_blobs: i32,

    /// The count of imported blobs.
    #[serde(rename = "importedBlobs", default)]
    pub imported_blobs: i32
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

/// Represents an app password creation response.
///
/// [`com.atproto.server.createAppPassword#responses`](https://docs.bsky.app/docs/api/com-atproto-server-create-app-password#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAppPasswordResponse {
    /// A short name for the App Password, to help distinguish them.
    #[serde(rename = "name")]
    pub name: String,

    /// The app password.
    #[serde(rename = "password")]
    pub password: String,

    /// The date and time the app password was created.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    /// Whether the app password has 'privileged' access to possibly sensitive
    /// account data.
    #[serde(rename = "privileged", default)]
    pub privileged: bool
}

/// Represents an invite code response.
///
/// [`com.atproto.server.createInviteCode#responses`](https://docs.bsky.app/docs/api/com-atproto-server-create-invite-code#responses)
#[derive(Serialize, Deserialize)]
pub struct InviteCodeResponse {
    /// The invite code.
    #[serde(rename = "code")]
    pub code: String
}

impl std::fmt::Display for InviteCodeResponse {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

/// Represents a session creation response.
///
/// [`com.atproto.server.createSession#responses`](https://docs.bsky.app/docs/api/com-atproto-server-create-session#responses)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSessionResponse {
    /// The access JWT.
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,

    /// The refresh JWT.
    #[serde(rename = "refreshJwt")]
    pub refresh_jwt: String,

    /// The handle of the account.
    #[serde(rename = "handle")]
    pub handle: String,

    /// The DID of the account.
    #[serde(rename = "did")]
    pub did: String,

    /// DID document of the account.
    #[serde(rename = "didDoc", skip_serializing_if = "Option::is_none")]
    pub did_doc: Option<DidDoc>,

    /// The email of the account.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Whether the email is confirmed.
    #[serde(rename = "emailConfirmed", default)]
    pub email_confirmed: bool,

    /// Whether email is used as an authentication factor.
    #[serde(rename = "emailAuthFactor", default)]
    pub email_auth_factor: bool,

    /// Whether the session is active.
    #[serde(rename = "active", default)]
    pub active: bool,

    /// The status of the session.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>
}

/// Represents a server description response.
///
/// [`com.atproto.server.describeServer#responses`](https://docs.bsky.app/docs/api/com-atproto-server-describe-server#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct DescribeServerResponse {
    /// Whether the server requires an invite code to join.
    #[serde(rename = "inviteCodeRequired", default)]
    pub invite_code_required: bool,

    /// Whether the server requires phone verification.
    #[serde(rename = "phoneVerificationRequired", default)]
    pub phone_verification_required: bool,

    /// The available user domains to use for handles.
    #[serde(rename = "availableUserDomains")]
    pub available_user_domains: Vec<String>,

    /// Links to the server's privacy policy and terms of service.
    #[serde(rename = "links")]
    pub links: ServerLinks,

    /// Contact information for the server.
    #[serde(rename = "contact")]
    pub contact: ServerContact
}

/// Represents a service auth response.
///
/// [`com.atproto.server.getServiceAuth#responses`](https://docs.bsky.app/docs/api/com-atproto-server-get-service-auth#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceAuthResponse {
    /// The service auth token.
    #[serde(rename = "token")]
    pub token: String
}

/// Represents a session request.
///
/// [`com.atproto.server.getSession#request`](https://docs.bsky.app/docs/api/com-atproto-server-get-session#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct SessionResponse {
    /// The access JWT.
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,

    /// The refresh JWT.
    #[serde(rename = "refreshJwt")]
    pub refresh_jwt: String,

    /// The handle of the account.
    #[serde(rename = "handle")]
    pub handle: String,

    /// The DID of the account.
    #[serde(rename = "did")]
    pub did: String,

    /// The email of the account.
    #[serde(rename = "email")]
    pub email: String,

    /// Whether the email is confirmed.
    #[serde(rename = "emailConfirmed", default)]
    pub email_confirmed: bool,

    /// Whether email is used as an authentication factor.
    #[serde(rename = "emailAuthFactor", default)]
    pub email_auth_factor: bool,

    /// Whether the session is active.
    #[serde(rename = "active", default)]
    pub active: bool,

    /// The status of the session.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>
}

/// Represents a list app passwords response.
///
/// [`com.atproto.server.listAppPasswords#responses`](https://docs.bsky.app/docs/api/com-atproto-server-list-app-passwords#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct ListAppPasswordsResponse {
    /// The app passwords.
    #[serde(rename = "passwords")]
    pub passwords: Vec<AppPassword>
}

/// Represents a session refresh response.
///
/// [`com.atproto.server.refreshSession#responses`](https://docs.bsky.app/docs/api/com-atproto-server-refresh-session#responses)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefreshSessionResponse {
    /// The access JWT.
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,

    /// The refresh JWT.
    #[serde(rename = "refreshJwt")]
    pub refresh_jwt: String,

    /// The handle of the account.
    #[serde(rename = "handle")]
    pub handle: String,

    /// The DID of the account.
    #[serde(rename = "did")]
    pub did: String,

    /// DID document of the account.
    #[serde(rename = "didDoc", skip_serializing_if = "Option::is_none")]
    pub did_doc: Option<DidDoc>,

    /// Whether the session is active.
    #[serde(rename = "active", default)]
    pub active: bool,

    /// The status of the session.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>
}

/// Represents an email update response.
///
/// [`com.atproto.server.requestEmailUpdate#responses`](https://docs.bsky.app/docs/api/com-atproto-server-request-email-update#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestEmailUpdateResponse {
    /// Whether a token is required.
    #[serde(rename = "tokenRequired", default)]
    pub token_required: bool
}

/// Represents a signing key reservation response.
///
/// [`com.atproto.server.reserveSigningKey#responses`](https://docs.bsky.app/docs/api/com-atproto-server-reserve-signing-key#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct ReserveSigningKeyResponse {
    /// The reserved signing key.
    #[serde(rename = "signingKey")]
    pub signing_key: String
}
