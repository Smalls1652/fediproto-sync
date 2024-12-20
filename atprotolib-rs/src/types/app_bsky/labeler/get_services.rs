use serde::{Deserialize, Serialize};

use super::{LabelerView, LabelerViewDetailed};

/*
    app.bsky.labeler.getServices
*/

/// The response to a request getting services for a labeler.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetServicesResponse {
    /// The views.
    views: Vec<LabelerViewDetailed>
}

/// A type union for the views in the response.
#[derive(Serialize, Deserialize, Debug)]
pub enum GetServicesResponseViews {
    /// A view of a labeler.
    LabelerView(LabelerView),

    /// A detailed view of a labeler.
    LabelerViewDetailed(LabelerViewDetailed)
}
