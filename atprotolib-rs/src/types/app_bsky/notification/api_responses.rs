use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::Notification;

/// The response to a request for the unread count of notifications.
#[derive(Serialize, Deserialize, Debug)]
pub struct UnreadCountResponse {
    /// The count of unread notifications.
    #[serde(rename = "count", default)]
    pub count: i32
}

/// The response to a request for a user's notifications.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListNotificationsResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's notifications.
    notifications: Vec<Notification>,

    /// Whether the notifications are priority notifications.
    priority: bool,

    /// The date and time the notifications were last seen.
    seen_at: Option<DateTime<Utc>>
}
