use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The request to update the user's notification preferences.
#[derive(Serialize, Deserialize, Debug)]
pub struct PutPreferencesRequest {
    /// Whether to enable only priority notifications.
    #[serde(rename = "priority", default)]
    pub priority: bool
}

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

/// The request to update the time at which notifications were seen.
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSeenRequest {
    /// The date and time when the notification was seen at.
    #[serde(rename = "seenAt")]
    pub seen_at: DateTime<Utc>
}
