use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

/*
    app.bsky.graph.getFollows
*/

/// The response to a request for the profiles a user follows.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetFollowsResponse {
    /// The subject of the request.
    subject: ProfileView,

    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the profiles the user follows.
    follows: Vec<ProfileView>
}
