use serde::{Deserialize, Serialize};

use super::InviteCode;

/*
    com.atproto.server.getAccountInviteCodes
*/

/// Represents a request to get account invite codes.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccountInviteCodesRequest {
    /// Codes to get.
    #[serde(rename = "codes")]
    pub codes: Vec<InviteCode>
}
