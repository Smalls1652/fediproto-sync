use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

/*
    app.bsky.graph.getFollowers
*/

/// The response to a request for a user's followers.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetFollowersResponse {
    /// The subject of the request.
    subject: ProfileView,

    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's followers.
    followers: Vec<ProfileView>
}
