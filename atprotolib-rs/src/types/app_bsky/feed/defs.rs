use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    app_bsky::{
        actor::{ProfileView, ProfileViewBasic},
        embed::{
            external::ExternalEmbedView,
            image::ImageEmbedView,
            record::RecordEmbedView,
            record_with_media::RecordWithMediaEmbedView,
            video::VideoEmbedView
        },
        graph::ListViewBasic
    },
    com_atproto::label::Label
};

/// Represents a post view.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#postView")]
pub struct PostView {
    /// The URI of the post.
    #[serde(rename = "uri")]
    pub uri: String,

    /// The content ID of the post.
    #[serde(rename = "cid")]
    pub cid: String,

    /// The profile of the author.
    #[serde(rename = "author")]
    pub author: ProfileViewBasic,

    /// The record of the post.
    #[serde(rename = "record")]
    pub record: serde_json::Value,

    /// Embed information for the post.
    #[serde(rename = "embed", skip_serializing_if = "Option::is_none")]
    pub embed: Option<PostViewEmbed>,

    /// The number of replies to the post.
    #[serde(rename = "replyCount", default)]
    pub reply_count: i32,

    /// The number of reposts of the post.
    #[serde(rename = "repostCount", default)]
    pub repost_count: i32,

    /// The number of likes on the post.
    #[serde(rename = "likeCount", default)]
    pub like_count: i32,

    /// The number of quotes of the post.
    #[serde(rename = "quoteCount", default)]
    pub quote_count: i32,

    /// The date and time the post was indexed.
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>,

    /// Information about the viewer's state of the post.
    #[serde(rename = "viewer", skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,

    /// Labels for the post.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,

    /// Unknown.
    #[serde(rename = "threadgate", skip_serializing_if = "Option::is_none")]
    pub threadgate: Option<ThreadgateView>
}

/// Represents an embed for a post view.
#[derive(Serialize, Deserialize, Debug)]
pub enum PostViewEmbed {
    /// An image embed.
    Images(ImageEmbedView),

    /// A video embed.
    Video(VideoEmbedView),

    /// An external embed.
    External(ExternalEmbedView),

    /// A record embed.
    Record(RecordEmbedView),

    /// A record with media embed.
    RecordWithMedia(RecordWithMediaEmbedView)
}

/// Metadata about the requesting account's relationship with the subject
/// content. Only has meaningful content for authed requests.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#viewerState")]
pub struct ViewerState {
    /// ATProto URI of a repost. (?)
    #[serde(rename = "repost", skip_serializing_if = "Option::is_none")]
    pub repost: Option<String>,

    /// ATProto URI of a like. (?)
    #[serde(rename = "like", skip_serializing_if = "Option::is_none")]
    pub like: Option<String>,

    /// Whether the thread is muted.
    #[serde(rename = "threadMuted", default)]
    pub thread_muted: bool,

    /// Whether replies are disabled.
    #[serde(rename = "replyDisabled", default)]
    pub reply_disabled: bool,

    /// Whether embedding is disabled.
    #[serde(rename = "embeddingDisabled", default)]
    pub embedding_disabled: bool,

    /// Whether the post is pinned.
    #[serde(rename = "pinned", default)]
    pub pinned: bool
}

/// Represents a post in a feed view.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#feedViewPost")]
pub struct FeedViewPost {
    /// The post.
    #[serde(rename = "post")]
    pub post: PostView,

    /// The parent post the post is replying to.
    #[serde(rename = "reply", skip_serializing_if = "Option::is_none")]
    pub reply: Option<ReplyRef>,

    /// The reason for the post.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<FeedViewPostReason>,

    /// Context provided by a feed generator that may be passed back alongside
    /// interactions.
    #[serde(rename = "feedContext", skip_serializing_if = "Option::is_none")]
    pub feed_context: Option<String>
}

/// Represents a reason for a post in a feed view.
#[derive(Serialize, Deserialize, Debug)]
pub enum FeedViewPostReason {
    /// The post is a repost.
    Repost(ReasonRepost),

    /// The post is pinned.
    Pin(ReasonPin)
}

/// Represents a reference to a reply.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#replyRef")]
pub struct ReplyRef {
    /// The root post.
    #[serde(rename = "root")]
    pub root: ReplyRefItem,

    /// The parent post.
    #[serde(rename = "parent")]
    pub parent: ReplyRefItem,

    /// If the parent reply is to another post, this is the author of the
    /// original post.
    #[serde(rename = "grandparentAuthor", skip_serializing_if = "Option::is_none")]
    pub grandparent_author: Option<ProfileViewBasic>
}

/// Represents an item in a reply reference.
#[derive(Serialize, Deserialize, Debug)]
pub enum ReplyRefItem {
    /// A post.
    Post(PostView),

    /// The post was not found.
    NotFoundPost(NotFoundPost),

    /// The post is blocked.
    BlockedPost(BlockedPost)
}

/// Represents a reason for a repost.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#reasonRepost")]
pub struct ReasonRepost {
    /// The profile of the user who reposted the post.
    #[serde(rename = "by")]
    pub by: ProfileViewBasic,

    /// The date and time the post was indexed.
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>
}

/// Represents a reason for a pin.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#reasonPin")]
pub struct ReasonPin {}

/// Represents a post in a thread view.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#threadViewPost")]
pub struct ThreadViewPost {
    /// The post.
    #[serde(rename = "post")]
    pub post: PostView,

    /// The parent of the post in the thread.
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<ThreadViewPostItem>,

    /// Replies to the post.
    #[serde(rename = "replies", skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<ThreadViewPostItem>>
}

/// Represents an item in a thread view post.
#[derive(Serialize, Deserialize, Debug)]
pub enum ThreadViewPostItem {
    /// A post.
    ThreadPost(Box<ThreadViewPost>),

    /// The post was not found.
    NotFoundPost(NotFoundPost),

    /// The post is blocked.
    BlockedPost(BlockedPost)
}

/// Represents a post not found.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#notFoundPost")]
pub struct NotFoundPost {
    /// The URI of the post.
    #[serde(rename = "uri")]
    pub uri: String,

    /// Whether the post was not found.
    #[serde(rename = "notFound", default)]
    pub not_found: bool
}

/// Represents a blocked post.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#blockedPost")]
pub struct BlockedPost {
    /// The URI of the post.
    #[serde(rename = "uri")]
    pub uri: String,

    /// Whether the post is blocked.
    #[serde(rename = "blocked", default)]
    pub blocked: bool,

    /// The author of the post that is blocked.
    #[serde(rename = "author")]
    pub author: BlockedAuthor
}

/// Represents a blocked author.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#blockedAuthor")]
pub struct BlockedAuthor {
    /// The DID of the blocked author.
    #[serde(rename = "did")]
    pub did: String,

    /// The relationship of the blocked author.
    #[serde(rename = "viewer", skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>
}

/*    Type: generatorView
    Id: app.bsky.feed.defs#generatorView
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
    - cid: string (JsonProperty: cid) [Required]
    - did: string (JsonProperty: did) [Required]
    - creator: app.bsky.actor.defs#profileView (JsonProperty: creator) [Required]
    - display_name: string (JsonProperty: displayName) [Required]
    - description: string (JsonProperty: description) [Optional]
    - description_facets: app.bsky.richtext.facet[] (JsonProperty: descriptionFacets) [Optional]
    - avatar: string (JsonProperty: avatar) [Optional]
    - like_count: integer  (JsonProperty: likeCount) [Optional]
    - accepts_interactions: boolean  (JsonProperty: acceptsInteractions) [Optional]
    - labels: com.atproto.label.defs#label[] (JsonProperty: labels) [Optional]
    - viewer: #generatorViewerState (JsonProperty: viewer) [Optional]
    - indexed_at: datetime (JsonProperty: indexedAt) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#generatorView")]
pub struct GeneratorView {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "creator")]
    pub creator: ProfileView,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "descriptionFacets", skip_serializing_if = "Option::is_none")]
    pub description_facets: Option<Vec<serde_json::Value>>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "likeCount", default)]
    pub like_count: i32,
    #[serde(rename = "acceptsInteractions", default)]
    pub accepts_interactions: bool,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(rename = "viewer", skip_serializing_if = "Option::is_none")]
    pub viewer: Option<GeneratorViewerState>,
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>
}

/*    Type: generatorViewerState
    Id: app.bsky.feed.defs#generatorViewerState
    Kind: object

    Properties:
    - like: string (JsonProperty: like) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#generatorViewerState")]
pub struct GeneratorViewerState {
    #[serde(rename = "like", skip_serializing_if = "Option::is_none")]
    pub like: Option<String>
}

/*    Type: skeletonFeedPost
    Id: app.bsky.feed.defs#skeletonFeedPost
    Kind: object

    Properties:
    - post: string (JsonProperty: post) [Required]
    - reason: union  (JsonProperty: reason) [Optional]
    - feed_context: string (JsonProperty: feedContext) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#skeletonFeedPost")]
pub struct SkeletonFeedPost {
    #[serde(rename = "post")]
    pub post: String,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<SkeletonFeedPostReason>,
    #[serde(rename = "feedContext", skip_serializing_if = "Option::is_none")]
    pub feed_context: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SkeletonFeedPostReason {
    SkeletonReasonRepost(SkeletonReasonRepost),
    SkeletonReasonPin(SkeletonReasonPin)
}

/*    Type: skeletonReasonRepost
    Id: app.bsky.feed.defs#skeletonReasonRepost
    Kind: object

    Properties:
    - repost: string (JsonProperty: repost) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#skeletonReasonRepost")]
pub struct SkeletonReasonRepost {
    #[serde(rename = "repost")]
    pub repost: String
}

/*    Type: skeletonReasonPin
    Id: app.bsky.feed.defs#skeletonReasonPin
    Kind: object

    Properties:
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#skeletonReasonPin")]
pub struct SkeletonReasonPin {}

/*    Type: threadgateView
    Id: app.bsky.feed.defs#threadgateView
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Optional]
    - cid: string (JsonProperty: cid) [Optional]
    - record: unknown  (JsonProperty: record) [Optional]
    - lists: app.bsky.graph.defs#listViewBasic[] (JsonProperty: lists) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#threadgateView")]
pub struct ThreadgateView {
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "record", skip_serializing_if = "Option::is_none")]
    pub record: Option<serde_json::Value>,
    #[serde(rename = "lists", skip_serializing_if = "Option::is_none")]
    pub lists: Option<Vec<ListViewBasic>>
}

/*    Type: interaction
    Id: app.bsky.feed.defs#interaction
    Kind: object

    Properties:
    - item: string (JsonProperty: item) [Optional]
    - event: string (JsonProperty: event) [Optional]
    - feed_context: string (JsonProperty: feedContext) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.feed.defs#interaction")]
pub struct Interaction {
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(rename = "feedContext", skip_serializing_if = "Option::is_none")]
    pub feed_context: Option<String>
}
