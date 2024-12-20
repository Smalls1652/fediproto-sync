use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    app_bsky::{
        actor::ProfileViewBasic,
        feed::{BlockedAuthor, GeneratorView},
        graph::{ListView, StarterPackViewBasic},
        labeler::LabelerView
    },
    com_atproto::label::Label
};

/// A view of a record embed.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.record#view")]
pub struct RecordEmbedView {
    /// The embedded record.
    #[serde(rename = "record")]
    pub record: RecordEmbedUnion
}

/// Represents a union of different types of record embeds.
#[derive(Serialize, Deserialize, Debug)]
pub enum RecordEmbedUnion {
    /// A record embed.
    RecordEmbedViewRecord(RecordEmbedViewRecord),

    /// A record embed that was not found.
    RecordEmbedViewNotFound(RecordEmbedViewNotFound),

    /// A record embed that was blocked.
    RecordEmbedViewBlocked(RecordEmbedViewBlocked),

    /// A record embed that was detached.
    RecordEmbedViewDetached(RecordEmbedViewDetached),

    /// A generator view.
    GeneratorView(GeneratorView),

    /// A list view.
    ListView(ListView),

    /// A starter pack view.
    StarterPackViewBasic(StarterPackViewBasic),
    
    /// A labeler view.
    LabelerView(LabelerView)
}

/// A representation of a record embedded in a Bluesky record (eg, a post).
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.record#viewRecord")]
pub struct RecordEmbedViewRecord {
    /// The URI of the record.
    #[serde(rename = "uri")]
    pub uri: String,

    /// The CID of the record.
    #[serde(rename = "cid")]
    pub cid: String,

    /// The author of the record.
    #[serde(rename = "author")]
    pub author: ProfileViewBasic,

    /// The value of the record.
    #[serde(rename = "value")]
    pub value: serde_json::Value,

    /// Labels associated with the record.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,

    /// The number of replies to the record.
    #[serde(rename = "replyCount", default)]
    pub reply_count: i32,

    /// The number of reposts of the record.
    #[serde(rename = "repostCount", default)]
    pub repost_count: i32,

    /// The number of likes on the record.
    #[serde(rename = "likeCount", default)]
    pub like_count: i32,

    /// The number of quotes of the record.
    #[serde(rename = "quoteCount", default)]
    pub quote_count: i32,

    /// Embeds in the record.
    #[serde(rename = "embeds", skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<RecordEmbedUnion>>,

    /// The date and time the record was indexed.
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>
}

/// Represents a record embed that was not found.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.record#viewNotFound")]
pub struct RecordEmbedViewNotFound {
    /// The URI of the record.
    #[serde(rename = "uri")]
    pub uri: String,

    /// Whether the record was not found.
    #[serde(rename = "notFound", default)]
    pub not_found: bool
}

/// Represents a record embed that was blocked.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.record#viewBlocked")]
pub struct RecordEmbedViewBlocked {
    /// The URI of the record.
    #[serde(rename = "uri")]
    pub uri: String,

    /// Whether the record was blocked.
    #[serde(rename = "blocked", default)]
    pub blocked: bool,

    /// The blocked author of the record.
    #[serde(rename = "author")]
    pub author: BlockedAuthor
}

/// Represents a record embed that was detached.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.embed.record#viewDetached")]
pub struct RecordEmbedViewDetached {
    /// The URI of the record.
    #[serde(rename = "uri")]
    pub uri: String,

    /// Whether the record was detached.
    #[serde(rename = "detached", default)]
    pub detached: bool
}
