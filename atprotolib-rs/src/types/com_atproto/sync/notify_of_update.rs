use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.notifyOfUpdate
*/

/// Represents a request to notify of an update.
#[derive(Serialize, Deserialize, Debug)]
pub struct NotifyOfUpdateRequest {
    /// The hostname of the server.
    #[serde(rename = "hostname")]
    pub hostname: String
}
