use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/*
    com.atproto.server.listAppPasswords
*/

/// Represents a list app passwords response.
///
/// [`com.atproto.server.listAppPasswords#responses`](https://docs.bsky.app/docs/api/com-atproto-server-list-app-passwords#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct ListAppPasswordsResponse {
    /// The app passwords.
    #[serde(rename = "passwords")]
    pub passwords: Vec<ListAppPasswordsResponseAppPassword>
}

/// Represents an App Password.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListAppPasswordsResponseAppPassword {
    /// A short name for the App Password, to help distinguish them.
    #[serde(rename = "name")]
    pub name: String,

    /// The date and time the app password was created.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    /// If an app password has 'privileged' access to possibly sensitive account data.
    #[serde(rename = "privileged", default)]
    pub privileged: bool
}
