pub mod api_responses;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    app_bsky::actor::ProfileView,
    com_atproto::label::{Label, LabelValueDefinition}
};

/// A view of a labeler.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.labeler.defs#labelerView")]
pub struct LabelerView {
    /// The URI of the labeler.
    uri: String,

    /// The CID of the labeler.
    cid: String,

    /// The creator of the labeler.
    creator: ProfileView,

    /// The number of likes the labeler has.
    like_count: i32,

    /// A representation of the viewer's relationship with the labeler.
    viewer: Option<LabelerViewerState>,

    /// The date and time the labeler was indexed.
    indexed_at: DateTime<Utc>,

    /// The labels associated with the labeler.
    labels: Option<Vec<Label>>
}

/// A detailed view of a labeler.
#[derive(Serialize, Deserialize, Debug)]
pub struct LabelerViewDetailed {
    /// The URI of the labeler.
    uri: String,

    /// The CID of the labeler.
    cid: String,

    /// The creator of the labeler.
    creator: ProfileView,

    /// The policies of the labeler.
    policies: LabelerPolicies,

    /// The number of likes the labeler has.
    like_count: i32,

    /// A representation of the viewer's relationship with the labeler.
    viewer: Option<LabelerViewerState>,

    /// The date and time the labeler was indexed.
    indexed_at: DateTime<Utc>,

    /// The labels associated with the labeler.
    labels: Option<Vec<Label>>
}

/// A representation of the viewer's relationship with a labeler.
#[derive(Serialize, Deserialize, Debug)]
pub struct LabelerViewerState {
    /// The URI of the like record.
    like: Option<String>
}

/// Policies for a labeler.
#[derive(Serialize, Deserialize, Debug)]
pub struct LabelerPolicies {
    /// The label values of the labeler.
    label_values: Vec<String>,

    /// The label value definitions of the labeler.
    label_value_definitions: Option<Vec<LabelValueDefinition>>
}
