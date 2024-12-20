use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/*
    com.atproto.admin.getInviteCodes
*/

/// Represents a response to get invite codes.
///
/// [`com.atproto.admin.getInviteCodes#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-invite-codes#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct GetInviteCodesResponse {
    /// The cursor stream position.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// The invite codes.
    #[serde(rename = "codes")]
    pub codes: Vec<GetInviteCodesResponseCode>
}

#[allow(missing_docs)] // This should be replaced with just the response being `InviteCode`.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetInviteCodesResponseCode {
    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "available", default)]
    pub available: i32,

    #[serde(rename = "disabled", default)]
    pub disabled: bool,

    #[serde(rename = "forAccount")]
    pub for_account: String,

    #[serde(rename = "createdBy")]
    pub created_by: String,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "uses")]
    pub uses: Vec<GetInviteCodesResponseCodeUse>
}

#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetInviteCodesResponseCodeUse {
    #[serde(rename = "usedBy")]
    pub used_by: String,

    #[serde(rename = "usedAt")]
    pub used_at: DateTime<Utc>
}
