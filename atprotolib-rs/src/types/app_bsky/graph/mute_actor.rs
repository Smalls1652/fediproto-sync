use serde::{Deserialize, Serialize};

/*
    app.bsky.graph.muteActor
*/

/// The request to mute an actor.
#[derive(Serialize, Deserialize, Debug)]
pub struct MuteActorRequest {
    /// The URI of the actor to mute.
    actor: String
}
