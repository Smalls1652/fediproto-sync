use serde::{Deserialize, Serialize};

use crate::types::app_bsky::actor::ProfileView;

use super::{ListItemView, ListView, NotFoundActor, Relationship, StarterPackView, StarterPackViewBasic};

/// The response to a request for a user's starter packs.
#[derive(Serialize, Deserialize, Debug)]
pub struct ActorStarterPacksResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's starter packs.
    starter_packs: Vec<StarterPackViewBasic>
}

/// The response to a request for a user's blocked profiles.
#[derive(Serialize, Deserialize, Debug)]
pub struct BlocksResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's blocked profiles.
    blocks: Vec<ProfileView>
}

/// The response to a request for a user's followers.
#[derive(Serialize, Deserialize, Debug)]
pub struct FollowersResponse {
    /// The subject of the request.
    subject: ProfileView,

    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's followers.
    followers: Vec<ProfileView>
}

/// The response to a request for the profiles a user follows.
#[derive(Serialize, Deserialize, Debug)]
pub struct FollowsResponse {
    /// The subject of the request.
    subject: ProfileView,

    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the profiles the user follows.
    follows: Vec<ProfileView>
}

/// The response to a request for a user's known followers.
#[derive(Serialize, Deserialize, Debug)]
pub struct KnownFollowersResponse {
    /// The subject of the request.
    subject: ProfileView,

    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's known followers.
    followers: Vec<ProfileView>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListBlocksResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's blocked lists.
    lists: Vec<ListView>
}

/// The response to a request for a user's mute lists.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListMutesResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's mute lists.
    lists: Vec<ListView>
}

/// The response to a request for a list.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// The list.
    list: ListView,

    /// A list of the list's items.
    items: Vec<ListItemView>
}

/// The response to a request for lists.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListsResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the lists.
    lists: Vec<ListView>
}

/// The response to a request for a user's muted profiles.
#[derive(Serialize, Deserialize, Debug)]
pub struct MutesResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the user's muted profiles.
    mutes: Vec<ProfileView>
}

/// The response to a request for a user's relationships.
#[derive(Serialize, Deserialize, Debug)]
pub struct RelationshipsResponse {
    /// The account the relationships are for.
    actor: Option<String>,

    /// A list of the user's relationships.
    relationships: Vec<RelationshipsResponseRelationships>
}

/// A type union for the relationships.
#[derive(Serialize, Deserialize, Debug)]
pub enum RelationshipsResponseRelationships {
    /// A relationship.
    Relationship(Relationship),

    /// An actor that was not found.
    NotFoundActor(NotFoundActor)
}

/// The response to a request for a starter pack.
#[derive(Serialize, Deserialize, Debug)]
pub struct StarterPackResponse {
    /// The starter pack.
    starter_pack: StarterPackView
}

/// The response to a request for starter packs.
#[derive(Serialize, Deserialize, Debug)]
pub struct StarterPacksResponse {
    /// A list of the starter packs.
    starter_packs: Vec<StarterPackViewBasic>
}

/// The response to a request for suggested follows for a user.
#[derive(Serialize, Deserialize, Debug)]
pub struct SuggestedFollowsByActorResponse {
    /// A list of suggested profiles.
    suggestions: Vec<ProfileView>,

    /// Whether the response is a fallback.
    is_fallback: bool
}

/// The response to a request to search for starter packs.
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchStarterPacksResponse {
    /// The cursor for the stream.
    cursor: Option<String>,

    /// A list of the starter packs.
    starter_packs: Vec<StarterPackViewBasic>
}
