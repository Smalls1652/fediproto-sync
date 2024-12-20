use serde::{Deserialize, Serialize};

use super::ProfileViewBasic;

/*
    app.bsky.actor.searchActorsTypeahead
*/

/// A response for the typeahead search for actors.
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchActorsTypeaheadResponse {
    /// A list of actors.
    #[serde(rename = "actors")]
    pub actors: Vec<ProfileViewBasic>
}
