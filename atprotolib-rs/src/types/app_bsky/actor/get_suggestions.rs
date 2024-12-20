use serde::{Deserialize, Serialize};

use super::ProfileView;

/*
    app.bsky.actor.getSuggestions
*/

/// A response to getting suggested actors.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetSuggestionsResponse {
    /// A cursor for the stream.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// A list of actors.
    #[serde(rename = "actors")]
    pub actors: Vec<ProfileView>
}
