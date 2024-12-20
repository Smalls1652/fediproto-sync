use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.sendEmail
*/

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

/// Represents a response to a request to send an email.
///
/// [`com.atproto.admin.sendEmail#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-send-email#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct SendEmailResponse {
    /// Whether the email was sent.
    #[serde(rename = "sent", default)]
    pub sent: bool
}
