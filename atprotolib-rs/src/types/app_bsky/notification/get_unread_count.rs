use serde::{Deserialize, Serialize};

/*
    app.bsky.notification.getUnreadCount
*/

/// The response to a request for the unread count of notifications.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetUnreadCountResponse {
    /// The count of unread notifications.
    #[serde(rename = "count", default)]
    pub count: i32
}
