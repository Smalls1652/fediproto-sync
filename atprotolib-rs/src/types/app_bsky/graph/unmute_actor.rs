use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.unmuteActor
*/

/// The request to unmute an actor.
#[derive(Serialize, Deserialize, Debug)]
pub struct UnmuteActorRequest {
    /// The URI of the actor to unmute.
    actor: String
}
