use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::InviteCode;

/// Represents a confirm email request.
///
/// [`com.atproto.server.confirmEmail#request`](https://docs.bsky.app/docs/api/com-atproto-server-confirm-email#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfirmEmailRequest {
    /// The email to confirm.
    #[serde(rename = "email")]
    pub email: String,

    /// The confirmation token.
    #[serde(rename = "token")]
    pub token: String
}

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

/// Represents an app password creation request.
///
/// [`com.atproto.server.createAppPassword#request`](https://docs.bsky.app/docs/api/com-atproto-server-create-app-password#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAppPasswordRequest {
    /// A short name for the App Password, to help distinguish them.
    #[serde(rename = "name")]
    pub name: String,

    /// If an app password has 'privileged' access to possibly sensitive account
    /// state. Meant for use with trusted clients.
    #[serde(rename = "privileged", default)]
    pub privileged: bool
}

/// Represents an invite code request.
///
/// [`com.atproto.server.createInviteCode#request`](https://docs.bsky.app/docs/api/com-atproto-server-create-invite-code#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct InviteCodeRequest {
    /// The amount of uses the invite code has.
    #[serde(rename = "useCount", default)]
    pub use_count: i32,

    /// Generated for a specific account.
    #[serde(rename = "forAccount", skip_serializing_if = "Option::is_none")]
    pub for_account: Option<String>
}

/// Represents a session creation request.
///
/// [`com.atproto.server.createSession#request`](https://docs.bsky.app/docs/api/com-atproto-server-create-session#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSessionRequest {
    /// Handle or other identifier supported by the server for the
    /// authenticating user.
    #[serde(rename = "identifier")]
    pub identifier: String,

    /// The password of the authenticating user.
    #[serde(rename = "password")]
    pub password: String,

    /// An optional token for two-factor authentication.
    #[serde(rename = "authFactorToken", skip_serializing_if = "Option::is_none")]
    pub auth_factor_token: Option<String>
}

/// Represents an account deactivation request.
///
/// [`com.atproto.server.deactivateAccount#request`](https://docs.bsky.app/docs/api/com-atproto-server-deactivate-account#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct DeactivateAccountRequest {
    /// A recommendation to server as to how long they should hold onto the
    /// deactivated account before deleting.
    #[serde(rename = "deleteAfter")]
    pub delete_after: DateTime<Utc>
}

/// Represents an account deletion request.
///
/// [`com.atproto.server.deleteAccount#request`](https://docs.bsky.app/docs/api/com-atproto-server-delete-account#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteAccountRequest {
    /// The account's DID.
    #[serde(rename = "did")]
    pub did: String,

    /// The account's password.
    #[serde(rename = "password")]
    pub password: String,

    /// Confirmation token for the account deletion.
    #[serde(rename = "token")]
    pub token: String
}

/// Represents a request to get account invite codes.
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountInviteCodesRequest {
    /// Codes to get.
    #[serde(rename = "codes")]
    pub codes: Vec<InviteCode>
}

/// Represents a password reset request.
///
/// [`com.atproto.server.requestPasswordReset#request`](https://docs.bsky.app/docs/api/com-atproto-server-request-password-reset#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPasswordResetRequest {
    /// The email of the account to reset the password for.
    #[serde(rename = "email")]
    pub email: String
}

/// Represents a signing key reservation request.
///
/// [`com.atproto.server.reserveSigningKey#request`](https://docs.bsky.app/docs/api/com-atproto-server-reserve-signing-key#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct ReserveSigningKeyRequest {
    /// The DID of the account to reserve a signing key for.
    #[serde(rename = "did")]
    pub did: String
}

/// Represents a password reset request.
///
/// [`com.atproto.server.resetPassword#request`](https://docs.bsky.app/docs/api/com-atproto-server-reset-password#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct ResetPasswordRequest {
    /// A confirmation token.
    #[serde(rename = "token")]
    pub token: String,

    /// The current password.
    #[serde(rename = "password")]
    pub password: String
}

/// Represents an app password revocation request.
///
/// [`com.atproto.server.revokeAppPassword#request`](https://docs.bsky.app/docs/api/com-atproto-server-revoke-app-password#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct RevokeAppPasswordRequest {
    /// The name of the app password to revoke.
    #[serde(rename = "name")]
    pub name: String
}

/// Represents an email update request.
///
/// [`com.atproto.server.updateEmail#request`](https://docs.bsky.app/docs/api/com-atproto-server-update-email#request)
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateEmailRequest {
    /// The email address.
    #[serde(rename = "email")]
    pub email: String,

    /// Whether to require email authentication.
    #[serde(rename = "emailAuthFactor", default)]
    pub email_auth_factor: bool,

    /// Requires a token from `com.atproto.sever.requestEmailUpdate` if the
    /// account's email has been confirmed.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>
}
