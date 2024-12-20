use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    app_bsky::{
        actor::{ProfileView, ProfileViewBasic},
        feed::GeneratorView,
        richtext::RichTextFacet
    },
    com_atproto::label::Label
};

/// A basic view of a list.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListViewBasic {
    /// The URI of the list.
    uri: String,

    /// The CID of the list.
    cid: String,

    /// The name of the list.
    name: String,

    /// The purpose of the list.
    purpose: String,

    /// The URI for the avatar of the list.
    avatar: Option<String>,

    /// The number of items in the list.
    list_item_count: i32,

    /// The labels associated with the list.
    labels: Option<Vec<Label>>,

    /// A representation of the viewer's relationship with the list.
    viewer: Option<ListViewerState>,

    /// The date and time the list was indexed.
    indexed_at: Option<DateTime<Utc>>
}

/// A view of a list.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.graph.defs#listView")]
pub struct ListView {
    /// The URI of the list.
    uri: String,

    /// The CID of the list.
    cid: String,

    /// The creator of the list.
    creator: ProfileView,

    /// The name of the list.
    name: String,

    /// The purpose of the list.
    purpose: String,

    /// A description of the list.
    description: Option<String>,

    /// A description of the list, in rich text.
    description_facets: Option<Vec<RichTextFacet>>,

    /// The URI for the avatar of the list.
    avatar: Option<String>,

    /// The number of items in the list.
    list_item_count: i32,

    /// The labels associated with the list.
    labels: Option<Vec<Label>>,

    /// A representation of the viewer's relationship with the list.
    viewer: Option<ListViewerState>,

    /// The date and time the list was indexed.
    indexed_at: DateTime<Utc>
}

/// An item in a list.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListItemView {
    /// The URI of the profile.
    uri: String,

    /// The profile.
    subject: ProfileView
}

/// A view of a starter pack.
#[derive(Serialize, Deserialize, Debug)]
pub struct StarterPackView {
    /// The URI of the starter pack.
    uri: String,

    /// The CID of the starter pack.
    cid: String,

    /// The record of the starter pack.
    record: serde_json::Value,

    /// The creator of the starter pack.
    creator: ProfileViewBasic,

    /// The list associated with the starter pack.
    list: Option<ListViewBasic>,

    /// A sample of items in the list.
    list_items_sample: Option<Vec<ListItemView>>,

    /// Feed generators associated with the starter pack.
    feeds: Option<Vec<GeneratorView>>,

    /// Number of starter packs joined in the last week.
    joined_week_count: i32,

    /// Number of starter packs joined all time.
    joined_all_time_count: i32,

    /// The labels associated with the starter pack.
    labels: Option<Vec<Label>>,

    /// The date and time the starter pack was indexed.
    indexed_at: DateTime<Utc>
}

/// A basic view of a starter pack.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.graph.defs#starterPackViewBasic")]
pub struct StarterPackViewBasic {
    /// The URI of the starter pack.
    uri: String,

    /// The CID of the starter pack.
    cid: String,

    /// The record of the starter pack.
    record: serde_json::Value,

    /// The creator of the starter pack.
    creator: ProfileViewBasic,

    /// The count of items in the list.
    list_item_count: i32,

    /// The number of starter packs joined in the last week.
    joined_week_count: i32,

    /// The number of starter packs joined all time.
    joined_all_time_count: i32,

    /// The labels associated with the starter pack.
    labels: Option<Vec<Label>>,

    /// The date and time the starter pack was indexed.
    indexed_at: DateTime<Utc>
}

/// Represents the relationship between a viewer and a list.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.graph.defs#listViewerState")]
pub struct ListViewerState {
    /// Whether the list is muted.
    muted: bool,

    /// Whether the list is blocked.
    blocked: Option<String>
}

/// Represents an actor not found.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.graph.defs#notFoundActor")]
pub struct NotFoundActor {
    /// The URI of the actor.
    actor: String,

    /// Whether the actor is not found.
    not_found: bool
}

/// Represents a relationship.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.graph.defs#relationship")]
pub struct Relationship {
    /// The URI of the actor.
    did: String,

    /// If the actor follows this DID, this is the AT-URI of the follow record.
    following: Option<String>,

    /// If the actor is followed by this DID, contains the AT-URI of the follow record.
    followed_by: Option<String>
}
