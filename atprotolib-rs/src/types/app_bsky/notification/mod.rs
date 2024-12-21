pub mod api_requests;
pub mod api_responses;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{app_bsky::actor::ProfileView, com_atproto::label::Label};

/// Represents a notification.
#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    /// The URI of the notification.
    #[serde(rename = "uri")]
    pub uri: String,

    /// The CID of the notification.
    #[serde(rename = "cid")]
    pub cid: String,

    /// The author of the notification.
    #[serde(rename = "author")]
    pub author: ProfileView,

    /// The reason for the notification.
    #[serde(rename = "reason")]
    pub reason: String,

    /// The subject of the reason.
    #[serde(rename = "reasonSubject", skip_serializing_if = "Option::is_none")]
    pub reason_subject: Option<String>,

    /// The record associated with the notification.
    #[serde(rename = "record")]
    pub record: serde_json::Value,

    /// Whether the notification has been read.
    #[serde(rename = "isRead", default)]
    pub is_read: bool,

    /// The date and time the notification was indexed.
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>,

    /// Labels associated with the notification.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>
}
