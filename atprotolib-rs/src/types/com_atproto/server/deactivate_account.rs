use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/*
    com.atproto.server.deactivateAccount
*/

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
