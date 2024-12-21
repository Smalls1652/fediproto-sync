use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a response to a request to create a report.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateReportResponse {
    /// The ID of the report.
    #[serde(rename = "id")]
    pub id: i64,

    // TODO: Incorrect type for `reasonType`
    /// The type of reason for the report.
    #[serde(rename = "reasonType")]
    pub reason_type: String,

    /// The reason for the report.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// The subject of the report.
    #[serde(rename = "subject")]
    pub subject: serde_json::Value,

    /// The account that reported the subject.
    #[serde(rename = "reportedBy")]
    pub reported_by: String,

    /// The date and time the report was created.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>
}
