use serde::{Deserialize, Serialize};

/*
    app.bsky.notification.putPreferences
*/

/// The request to update the user's notification preferences.
#[derive(Serialize, Deserialize, Debug)]
pub struct PutPreferencesRequest {
    /// Whether to enable only priority notifications.
    #[serde(rename = "priority", default)]
    pub priority: bool
}
