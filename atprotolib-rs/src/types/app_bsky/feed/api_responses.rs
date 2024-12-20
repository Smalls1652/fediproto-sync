use serde::{Deserialize, Serialize};

use super::{
    BlockedPost,
    FeedGeneratorFeed,
    FeedGeneratorLinks,
    FeedViewPost,
    GeneratorView,
    Interaction,
    Like,
    NotFoundPost,
    PostView,
    ProfileView,
    SkeletonFeedPost,
    ThreadViewPost,
    ThreadgateView
};

/// The response to the `describeFeedGenerator` API call.
#[derive(Debug, Serialize, Deserialize)]
pub struct DescribeFeedGeneratorResponse {
    /// The DID of the feed generator.
    did: String,

    /// Feeds that the feed generator provides.
    feeds: Vec<FeedGeneratorFeed>,

    /// Links to the feed generator's privacy policy and terms of service.
    links: Option<FeedGeneratorLinks>
}

/// A response to getting feeds for an actor.
#[derive(Serialize, Deserialize, Debug)]
pub struct ActorFeedsResponse {
    /// A cursor for the stream.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// A list of feeds.
    #[serde(rename = "feeds")]
    pub feeds: Vec<GeneratorView>
}

/// A response to getting likes for an actor.
#[derive(Serialize, Deserialize, Debug)]
pub struct ActorLikesResponse {
    /// A cursor for the stream.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// A list of posts.
    #[serde(rename = "feed")]
    pub feed: Vec<FeedViewPost>
}

/// A response to getting a feed for an author.
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorFeedResponse {
    /// A cursor for the stream.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// A list of posts.
    #[serde(rename = "feed")]
    pub feed: Vec<FeedViewPost>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedGeneratorResponse {
    #[serde(rename = "view")]
    pub view: GeneratorView,
    #[serde(rename = "isOnline", default)]
    pub is_online: bool,
    #[serde(rename = "isValid", default)]
    pub is_valid: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedGeneratorsResponse {
    #[serde(rename = "feeds")]
    pub feeds: Vec<GeneratorView>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedSkeletonResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "feed")]
    pub feed: Vec<SkeletonFeedPost>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "feed")]
    pub feed: Vec<FeedViewPost>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LikesResponse {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "likes")]
    pub likes: Vec<Like>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFeedResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "feed")]
    pub feed: Vec<FeedViewPost>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostThreadResponse {
    #[serde(rename = "thread")]
    pub thread: PostThreadResponseThread,
    #[serde(rename = "threadgate", skip_serializing_if = "Option::is_none")]
    pub threadgate: Option<ThreadgateView>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PostThreadResponseThread {
    ThreadView(ThreadViewPost),
    NotFound(NotFoundPost),
    Blocked(BlockedPost)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostsResponse {
    #[serde(rename = "posts")]
    pub posts: Vec<PostView>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuotesResponse {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "posts")]
    pub posts: Vec<PostView>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepostedByResponse {
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "reposted_by")]
    pub reposted_by: Vec<ProfileView>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuggestedFeedsResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "feeds")]
    pub feeds: Vec<GeneratorView>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimelineResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "feed")]
    pub feed: Vec<FeedViewPost>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchPostsResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "hitsTotal", default)]
    pub hits_total: i32,
    #[serde(rename = "posts")]
    pub posts: Vec<PostView>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendInteractionsRequest {
    #[serde(rename = "interactions")]
    pub interactions: Vec<Interaction>
}
