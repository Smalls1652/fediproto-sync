use serde::{Deserialize, Serialize};

/// Represents an account deletion request.
///
/// [`com.atproto.admin.deleteAccount#request`](https://docs.bsky.app/docs/api/com-atproto-admin-delete-account#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAccountRequest {
    /// The DID of the account to delete.
    #[serde(rename = "did")]
    pub did: String
}

/// Represents a request to disable account invites.
///
/// [`com.atproto.admin.disableAccountInvites#request`](https://docs.bsky.app/docs/api/com-atproto-admin-disable-account-invites#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct DisableAccountInvitesRequest {
    /// The DID of the account.
    #[serde(rename = "account")]
    pub account: String,

    /// Optional reason for disabled invites.
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>
}

/// Represents a request to disable invite codes.
///
/// [`com.atproto.admin.disableInviteCodes#request`](https://docs.bsky.app/docs/api/com-atproto-admin-disable-invite-codes#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct DisableInviteCodesRequest {
    /// The invite codes to disable.
    #[serde(rename = "codes", skip_serializing_if = "Option::is_none")]
    pub codes: Option<Vec<String>>,

    /// The accounts to disable invite codes for.
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>
}

/// Represents a request to enable account invites.
///
/// [`com.atproto.admin.enableAccountInvites#request`](https://docs.bsky.app/docs/api/com-atproto-admin-enable-account-invites#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct EnableAccountInvitesRequest {
    /// The DID of the account.
    #[serde(rename = "account")]
    pub account: String,

    /// Optional reason for disabled invites.
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>
}

/// Represents a request to get account information.
///
/// [`com.atproto.admin.getAccountInfo#request`](https://docs.bsky.app/docs/api/com-atproto-admin-get-account-info#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfoRequest {
    /// The DID of the account to get information for.
    #[serde(rename = "did")]
    pub did: String
}

/// Represents a request to get account information for multiple accounts.
///
/// [`com.atproto.admin.getAccountInfos#request`](https://docs.bsky.app/docs/api/com-atproto-admin-get-account-infos#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfosRequest {
    /// The DIDs of the accounts to get information for.
    #[serde(rename = "did")]
    pub dids: Vec<String>
}

/// Represents a request to send an email.
///
/// [`com.atproto.admin.sendEmail#request`](https://docs.bsky.app/docs/api/com-atproto-admin-send-email#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct SendEmailRequest {
    /// The DID of the recipient.
    #[serde(rename = "recipientDid")]
    pub recipient_did: String,

    /// The content of the email.
    #[serde(rename = "content")]
    pub content: String,

    /// The subject of the email.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// The DID of the sender.
    #[serde(rename = "senderDid")]
    pub sender_did: String,

    /// An optional comment.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>
}

/// Represents a request to update an account's email.
///
/// [`com.atproto.admin.updateAccountEmail#request`](https://docs.bsky.app/docs/api/com-atproto-admin-update-account-email#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountEmailRequest {
    /// The DID of the account.
    #[serde(rename = "account")]
    pub account: String,

    /// The new email address.
    #[serde(rename = "email")]
    pub email: String
}

/// Represents a request to update an account's handle.
///
/// [`com.atproto.admin.updateAccountHandle#request`](https://docs.bsky.app/docs/api/com-atproto-admin-update-account-handle#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountHandleRequest {
    /// The DID of the account.
    #[serde(rename = "did")]
    pub did: String,

    /// The new handle.
    #[serde(rename = "handle")]
    pub handle: String
}

/// Represents a request to update an account's password.
///
/// [`com.atproto.admin.updateAccountPassword#request`](https://docs.bsky.app/docs/api/com-atproto-admin-update-account-password#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountPasswordRequest {
    /// The DID of the account.
    #[serde(rename = "did")]
    pub did: String,

    /// The new password.
    #[serde(rename = "password")]
    pub password: String
}
