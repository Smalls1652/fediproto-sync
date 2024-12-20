use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

/*
    app.bsky.graph.getSuggestedFollowsByActor
*/

/// The response to a request for suggested follows for a user.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetSuggestedFollowsByActorResponse {
    /// A list of suggested profiles.
    suggestions: Vec<ProfileView>,

    /// Whether the response is a fallback.
    is_fallback: bool
}
