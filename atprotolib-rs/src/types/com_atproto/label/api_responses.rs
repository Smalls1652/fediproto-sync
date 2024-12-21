use serde::{Deserialize, Serialize};

use super::Label;

/// Represents a response to a request to query labels.
#[derive(Serialize, Deserialize, Debug)]
pub struct QueryLabelsResponse {
    /// The cursor stream position.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// The labels that match the query criteria.
    #[serde(rename = "labels")]
    pub labels: Vec<Label>
}
