use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/*
    app.bsky.notification.updateSeen
*/

/// The request to update the time at which notifications were seen.
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSeenRequest {
    /// The date and time when the notification was seen at.
    #[serde(rename = "seenAt")]
    pub seen_at: DateTime<Utc>
}
