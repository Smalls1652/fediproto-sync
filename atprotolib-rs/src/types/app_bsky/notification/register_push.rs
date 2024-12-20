use serde::{Deserialize, Serialize};

/*
    app.bsky.notification.registerPush
*/

/// The request to register a push notification token.
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterPushRequest {
    /// The service DID.
    #[serde(rename = "serviceDid")]
    pub service_did: String,

    /// The push notification token.
    #[serde(rename = "token")]
    pub token: String,

    /// The platform of the push notification token.
    #[serde(rename = "platform")]
    pub platform: String,

    /// The app ID.
    #[serde(rename = "appId")]
    pub app_id: String
}
