use serde::{Deserialize, Serialize};

use crate::types::com_atproto::{admin::RepoRef, repo::StrongRef};

/// Represents a request to create a report.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateReportRequest {
    // TODO: Incorrect type for `reasonType`
    /// The type of reason for the report.
    #[serde(rename = "reasonType")]
    pub reason_type: String,

    /// The reason for the report.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// The subject of the report.
    #[serde(rename = "subject")]
    pub subject: CreateReportRequestSubject
}

/// Represents the subject of a report.
#[derive(Serialize, Deserialize, Debug)]
pub enum CreateReportRequestSubject {
    /// A repository reference.
    RepoRef(RepoRef),

    /// A strong reference.
    StrongRef(StrongRef)
}
