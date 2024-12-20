use serde::{Deserialize, Serialize};

use super::ProfileViewDetailed;

/*
    app.bsky.actor.getProfiles
*/

/// A response to getting multiple profiles.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetProfilesResponse {
    /// A list of profiles.
    #[serde(rename = "profiles")]
    pub profiles: Vec<ProfileViewDetailed>
}
