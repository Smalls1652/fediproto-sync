use serde::{Deserialize, Serialize};

use super::{NotFoundActor, Relationship};

/*
    app.bsky.graph.getRelationships
*/

/// The response to a request for a user's relationships.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetRelationshipsResponse {
    /// The account the relationships are for.
    actor: Option<String>,

    /// A list of the user's relationships.
    relationships: Vec<GetRelationshipsResponseRelationships>
}

/// A type union for the relationships.
#[derive(Serialize, Deserialize, Debug)]
pub enum GetRelationshipsResponseRelationships {
    /// A relationship.
    Relationship(Relationship),

    /// An actor that was not found.
    NotFoundActor(NotFoundActor)
}
