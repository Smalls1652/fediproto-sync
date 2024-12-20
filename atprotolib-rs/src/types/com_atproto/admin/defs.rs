use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::com_atproto::server::InviteCode;

/// Represents a status attribute.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.admin.defs#statusAttr")]
pub struct StatusAttr {
    /// Whether the status is applied.
    #[serde(rename = "applied", default)]
    pub applied: bool,

    /// The reference.
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>
}

/// Represents an account view.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.admin.defs#accountView")]
pub struct AccountView {
    /// The DID of the account.
    #[serde(rename = "did")]
    pub did: String,

    /// The handle of the account.
    #[serde(rename = "handle")]
    pub handle: String,

    /// The email associated with the account.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Records related to the account.
    #[serde(rename = "relatedRecords", skip_serializing_if = "Option::is_none")]
    pub related_records: Option<Vec<serde_json::Value>>,

    /// The date and time the account was indexed.
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>,

    /// The invite code used to create the account.
    #[serde(rename = "invitedBy", skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<InviteCode>,

    /// Invite codes the account has.
    #[serde(rename = "invites", skip_serializing_if = "Option::is_none")]
    pub invites: Option<Vec<InviteCode>>,

    /// Whether invites are disabled.
    #[serde(rename = "invitesDisabled", default)]
    pub invites_disabled: bool,

    /// The date and time the account's email was confirmed.
    #[serde(rename = "emailConfirmedAt", skip_serializing_if = "Option::is_none")]
    pub email_confirmed_at: Option<DateTime<Utc>>,

    /// An optional note about the invite.
    #[serde(rename = "inviteNote", skip_serializing_if = "Option::is_none")]
    pub invite_note: Option<String>,

    /// The date and time the account was deactivated.
    #[serde(rename = "deactivatedAt", skip_serializing_if = "Option::is_none")]
    pub deactivated_at: Option<DateTime<Utc>>,

    /// Threat signatures associated with the account.
    #[serde(rename = "threatSignatures", skip_serializing_if = "Option::is_none")]
    pub threat_signatures: Option<Vec<ThreatSignature>>
}

/// A reference to a repository.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.admin.defs#repoRef")]
pub struct RepoRef {
    /// The DID.
    #[serde(rename = "did")]
    pub did: String
}

/// A reference to a repository blob.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.admin.defs#repoBlobRef")]
pub struct RepoBlobRef {
    /// The DID.
    #[serde(rename = "did")]
    pub did: String,

    /// The CID.
    #[serde(rename = "cid")]
    pub cid: String,

    /// The URI for record.
    #[serde(rename = "recordUri", skip_serializing_if = "Option::is_none")]
    pub record_uri: Option<String>
}

/// Represents a threat signature.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.admin.defs#threatSignature")]
pub struct ThreatSignature {
    /// The property name of the threat signature.
    #[serde(rename = "property")]
    pub property: String,

    /// The value of the threat signature.
    #[serde(rename = "value")]
    pub value: String
}
