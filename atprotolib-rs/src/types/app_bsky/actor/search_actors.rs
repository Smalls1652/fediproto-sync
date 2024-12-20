use serde::{Deserialize, Serialize};

use super::ProfileView;

/*
    app.bsky.actor.searchActors
*/

/// A response to searching for actors.
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchActorsResponse {
    /// A cursor for the stream.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// A list of actors.
    #[serde(rename = "actors")]
    pub actors: Vec<ProfileView>
}
