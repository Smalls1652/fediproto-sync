use serde::{Deserialize, Serialize};

/*
    com.atproto.server.getServiceAuth
*/

/// Represents a service auth response.
///
/// [`com.atproto.server.getServiceAuth#responses`](https://docs.bsky.app/docs/api/com-atproto-server-get-service-auth#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct GetServiceAuthResponse {
    /// The service auth token.
    #[serde(rename = "token")]
    pub token: String
}
