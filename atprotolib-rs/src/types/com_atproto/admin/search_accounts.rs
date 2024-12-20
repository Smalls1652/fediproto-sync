use serde::{Deserialize, Serialize};

use super::GetAccountInfoResponse;

/*
    com.atproto.admin.searchAccounts
*/

/// Represents a response to a search for accounts.
///
/// [`com.atproto.admin.searchAccounts#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-search-accounts#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAccountsResponse {
    /// The cursor stream position.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// The accounts that match the search criteria.
    #[serde(rename = "accounts")]
    pub accounts: Vec<GetAccountInfoResponse>
}
