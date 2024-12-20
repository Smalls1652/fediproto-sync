use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

/*
    app.bsky.graph.getMutes
*/

/// The response to a request for a user's muted profiles.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetMutesResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's muted profiles.
    mutes: Vec<ProfileView>
}
