use serde::{Deserialize, Serialize};

use super::{ProfileView, ProfileViewBasic};

/// A response to getting suggested actors.
#[derive(Serialize, Deserialize, Debug)]
pub struct SuggestionsResponse {
    /// A cursor for the stream.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// A list of actors.
    #[serde(rename = "actors")]
    pub actors: Vec<ProfileView>
}

/// A response for the typeahead search for actors.
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchActorsTypeaheadResponse {
    /// A list of actors.
    #[serde(rename = "actors")]
    pub actors: Vec<ProfileViewBasic>
}

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
