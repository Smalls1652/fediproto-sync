use serde::{Deserialize, Serialize};

use super::get_account_info::GetAccountInfoResponse;

/*
    com.atproto.admin.getAccountInfos
*/

/// Represents a request to get account information for multiple accounts.
///
/// [`com.atproto.admin.getAccountInfos#request`](https://docs.bsky.app/docs/api/com-atproto-admin-get-account-infos#request)
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountInfosRequest {
    /// The DIDs of the accounts to get information for.
    #[serde(rename = "did")]
    pub dids: Vec<String>
}

/// Represents a response to a request to get account information for multiple
/// accounts.
///
/// [`com.atproto.admin.getAccountInfos#responses`](https://docs.bsky.app/docs/api/com-atproto-admin-get-account-infos#responses)
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountInfosResponse {
    /// Account information for the requested accounts.
    #[serde(rename = "infos")]
    pub infos: Vec<GetAccountInfoResponse>
}
