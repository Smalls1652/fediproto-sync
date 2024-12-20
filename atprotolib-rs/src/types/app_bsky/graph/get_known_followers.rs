use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

/*
    app.bsky.graph.getKnownFollowers
*/

/// The response to a request for a user's known followers.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetKnownFollowersResponse {
    /// The subject of the request.
    subject: ProfileView,

    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's known followers.
    followers: Vec<ProfileView>
}
