use serde::{Deserialize, Serialize};

/// The request to mute an actor list.
#[derive(Serialize, Deserialize, Debug)]
pub struct MuteActorListRequest {
    /// The list.
    list: String
}

/// The request to mute an actor.
#[derive(Serialize, Deserialize, Debug)]
pub struct MuteActorRequest {
    /// The URI of the actor to mute.
    actor: String
}

/// The request to mute a thread.
#[derive(Serialize, Deserialize, Debug)]
pub struct MuteThreadRequest {
    /// The URI of the thread to mute.
    root: String
}

/// The request to unmute an actor list.
#[derive(Serialize, Deserialize, Debug)]
pub struct UnmuteActorListRequest {
    /// The list.
    list: String
}

/// The request to unmute an actor.
#[derive(Serialize, Deserialize, Debug)]
pub struct UnmuteActorRequest {
    /// The URI of the actor to unmute.
    actor: String
}

/// The request to unmute a thread.
#[derive(Serialize, Deserialize, Debug)]
pub struct UnmuteThreadRequest {
    /// The URI of the thread to unmute.
    root: String
}
