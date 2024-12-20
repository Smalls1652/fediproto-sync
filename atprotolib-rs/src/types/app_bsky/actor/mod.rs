#[cfg(feature = "apicalls")]
pub mod api_calls;

pub mod api_responses;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    app_bsky::graph::{ListViewBasic, StarterPackViewBasic},
    com_atproto::label::Label
};

/// A view of a profile with basic information.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#profileViewBasic")]
pub struct ProfileViewBasic {
    /// The DID of the profile.
    #[serde(rename = "did")]
    pub did: String,

    /// The handle of the profile.
    #[serde(rename = "handle")]
    pub handle: String,

    /// The display name of the profile.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The avatar of the profile.
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    /// Profiles associated with this profile.
    #[serde(rename = "associated", skip_serializing_if = "Option::is_none")]
    pub associated: Option<ProfileAssociated>,

    /// The state of the profile relative to the viewer.
    #[serde(rename = "viewer", skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,

    /// Labels associated with the profile.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,

    /// The date and time the profile was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>
}

/// A view of a profile.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#profileView")]
pub struct ProfileView {
    /// The DID of the profile.
    #[serde(rename = "did")]
    pub did: String,

    /// The handle of the profile.
    #[serde(rename = "handle")]
    pub handle: String,

    /// The display name of the profile.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The description of the profile.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The avatar of the profile.
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    /// Profiles associated with this profile.
    #[serde(rename = "associated", skip_serializing_if = "Option::is_none")]
    pub associated: Option<ProfileAssociated>,

    /// The date and time the profile was indexed.
    #[serde(rename = "indexedAt", skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<DateTime<Utc>>,

    /// The date and time the profile was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// The state of the profile relative to the viewer.
    #[serde(rename = "viewer", skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,

    /// Labels associated with the profile.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>
}

/// A detailed view of a profile.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#profileViewDetailed")]
pub struct ProfileViewDetailed {
    /// The DID of the profile.
    #[serde(rename = "did")]
    pub did: String,

    /// The handle of the profile.
    #[serde(rename = "handle")]
    pub handle: String,

    /// The display name of the profile.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The description of the profile.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The avatar of the profile.
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    /// The banner of the profile.
    #[serde(rename = "banner", skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,

    /// The number of followers of the profile.
    #[serde(rename = "followersCount", default)]
    pub followers_count: i32,

    /// The number of profiles the profile follows.
    #[serde(rename = "followsCount", default)]
    pub follows_count: i32,

    /// The number of posts the profile has made.
    #[serde(rename = "postsCount", default)]
    pub posts_count: i32,

    /// Profiles associated with this profile.
    #[serde(rename = "associated", skip_serializing_if = "Option::is_none")]
    pub associated: Option<ProfileAssociated>,

    /// Data regarding the starter pack the profile joined via.
    #[serde(
        rename = "joinedViaStarterPack",
        skip_serializing_if = "Option::is_none"
    )]
    pub joined_via_starter_pack: Option<StarterPackViewBasic>,

    /// The date and time the profile was indexed.
    #[serde(rename = "indexedAt", skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<DateTime<Utc>>,

    /// The date and time the profile was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// The state of the profile relative to the viewer.
    #[serde(rename = "viewer", skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerState>,

    /// Labels associated with the profile.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,

    /// The pinned post of the profile.
    #[serde(rename = "pinnedPost", skip_serializing_if = "Option::is_none")]
    pub pinned_post: Option<String>
}

/// Data regarding a profile associated with a profile.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#profileAssociated")]
pub struct ProfileAssociated {
    /// The number of lists the profile is associated with.
    #[serde(rename = "lists", default)]
    pub lists: i32,

    /// The number of feed generators the profile is associated with.
    #[serde(rename = "feedgens", default)]
    pub feedgens: i32,

    /// The number of starter packs the profile is associated with.
    #[serde(rename = "starterPacks", default)]
    pub starter_packs: i32,

    /// The number of labelers the profile is associated with.
    #[serde(rename = "labeler", default)]
    pub labeler: bool,

    /// Information regarding chats associated with the profile.
    #[serde(rename = "chat", skip_serializing_if = "Option::is_none")]
    pub chat: Option<ProfileAssociatedChat>
}

/// Contains information regarding chats associated with a profile.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#profileAssociatedChat")]
pub struct ProfileAssociatedChat {
    #[serde(rename = "allowIncoming")]
    pub allow_incoming: String
}

/// Represents the state of profile relative to the viewer.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#viewerState")]
pub struct ViewerState {
    /// Whether the profile is muted.
    #[serde(rename = "muted", default)]
    pub muted: bool,

    /// Whether the profile is muted by a list.
    #[serde(rename = "mutedByList", skip_serializing_if = "Option::is_none")]
    pub muted_by_list: Option<ListViewBasic>,

    /// Whether the profile is blocked by the viewer. (?)
    #[serde(rename = "blockedBy", default)]
    pub blocked_by: bool,

    /// The ATProtocol URI of the block.
    #[serde(rename = "blocking")]
    pub blocking: String,

    /// Lists the profile is blocked by.
    #[serde(rename = "blockingByList", skip_serializing_if = "Option::is_none")]
    pub blocking_by_list: Option<ListViewBasic>,

    /// The ATProtocol URI of the follow.
    #[serde(rename = "following")]
    pub following: String,

    /// The ATProtocol URI for followed by. (?)
    #[serde(rename = "followedBy")]
    pub followed_by: String,

    /// Followers of the profile known to the viewer.
    #[serde(rename = "knownFollowers", skip_serializing_if = "Option::is_none")]
    pub known_followers: Option<KnownFollowers>
}

/// Represents followers of a profile known to the viewer.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#knownFollowers")]
pub struct KnownFollowers {
    /// The number of known followers.
    #[serde(rename = "count", default)]
    pub count: i32,

    /// Basic profile views of known followers.
    #[serde(rename = "followers")]
    pub followers: Vec<ProfileViewBasic>
}

/// Represents a profile's preferences.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#preferences")]
pub struct Preferences {
    #[serde(rename = "preferences")]
    pub preferences: Vec<PreferencesEnum>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PreferencesEnum {
    /// Adult content preferences.
    AdultContentPref(AdultContentPref),

    /// Content label preferences.
    ContentLabelPref(ContentLabelPref),

    /// Saved feeds (v2) preferences.
    SavedFeedsPrefV2(SavedFeedsPrefV2),

    /// Saved feeds preferences.
    SavedFeedsPref(SavedFeedsPref),

    /// Personal details preferences.
    PersonalDetailsPref(PersonalDetailsPref),

    /// Feed view preferences.
    FeedViewPref(FeedViewPref),

    /// Thread view preferences.
    ThreadViewPref(ThreadViewPref),

    /// Interests preferences.
    InterestsPref(InterestsPref),

    /// Muted words preferences.
    MutedWordsPref(MutedWordsPref),

    /// Hidden posts preferences.
    HiddenPostsPref(HiddenPostsPref),

    /// Labelers preferences.
    LabelersPref(LabelersPref)
}

/// Represents adult content preferences.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#adultContentPref")]
pub struct AdultContentPref {
    /// Whether adult content is enabled.
    #[serde(rename = "enabled", default)]
    pub enabled: bool
}

/// Represents content label preferences.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#contentLabelPref")]
pub struct ContentLabelPref {
    /// The DID of the labeler.
    #[serde(rename = "labelerDid", skip_serializing_if = "Option::is_none")]
    pub labeler_did: Option<String>,

    /// The name of the label.
    #[serde(rename = "label")]
    pub label: String,

    /// The visibility of the label.
    #[serde(rename = "visibility")]
    pub visibility: String
}

/// Represents a saved feed.
#[derive(Serialize, Deserialize, Debug)]
pub struct SavedFeed {
    /// The ID of the saved feed.
    #[serde(rename = "id")]
    pub id: String,

    /// The type of the saved feed.
    #[serde(rename = "type")]
    pub type_: String,

    /// The value of the saved feed.
    #[serde(rename = "value")]
    pub value: String,

    /// Whether the saved feed is pinned.
    #[serde(rename = "pinned", default)]
    pub pinned: bool
}

/// Represents saved feeds preferences.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#savedFeedsPrefV2")]
pub struct SavedFeedsPrefV2 {
    /// A list of saved feeds.
    #[serde(rename = "items")]
    pub items: Vec<SavedFeed>
}

/// Represents saved feeds preferences.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#savedFeedsPref")]
pub struct SavedFeedsPref {
    /// A list of pinned saved feeds.
    #[serde(rename = "pinned")]
    pub pinned: Vec<String>,

    /// A list of saved feeds.
    #[serde(rename = "saved")]
    pub saved: Vec<String>,

    /// The index of the timeline.
    #[serde(rename = "timelineIndex", default)]
    pub timeline_index: i32
}

/// Represents personal details preferences.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#personalDetailsPref")]
pub struct PersonalDetailsPref {
    /// The user's birth date.
    #[serde(rename = "birthDate", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<DateTime<Utc>>
}

/// Represents feed view preferences.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#feedViewPref")]
pub struct FeedViewPref {
    /// The URI of the feed.
    #[serde(rename = "feed")]
    pub feed: String,

    /// Whether replies are hidden.
    #[serde(rename = "hideReplies", default)]
    pub hide_replies: bool,

    /// Whether replies by unfollowed users are hidden.
    #[serde(rename = "hideRepliesByUnfollowed", default)]
    pub hide_replies_by_unfollowed: bool,

    /// How many likes are required for a reply to show in the feed.
    #[serde(rename = "hideRepliesByLikeCount", default)]
    pub hide_replies_by_like_count: i32,

    /// Whether reposts are hidden.
    #[serde(rename = "hideReposts", default)]
    pub hide_reposts: bool,

    /// Whether quote posts are hidden.
    #[serde(rename = "hideQuotePosts", default)]
    pub hide_quote_posts: bool
}

/// Represents thread view preferences.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#threadViewPref")]
pub struct ThreadViewPref {
    /// The sorting mode for threads.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,

    /// Whether to prioritize followed users.
    #[serde(rename = "prioritizeFollowedUsers", default)]
    pub prioritize_followed_users: bool
}

/// Represents interests preferences.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#interestsPref")]
pub struct InterestsPref {
    /// A list of tags which describe the account owner's interests gathered during onboarding.
    #[serde(rename = "tags")]
    pub tags: Vec<String>
}

/// Represents a muted word.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#mutedWord")]
pub struct MutedWord {
    /// The ID of the muted word.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The value of the muted word.
    #[serde(rename = "value")]
    pub value: String,

    /// The targets of the muted word.
    #[serde(rename = "targets")]
    pub targets: Vec<String>,

    /// Groups of users to apply the muted word to. If undefined, applies to all users.
    #[serde(rename = "actorTarget", skip_serializing_if = "Option::is_none")]
    pub actor_target: Option<String>,

    /// The date and time the muted word expires.
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>
}

/// Represents muted words preferences.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#mutedWordsPref")]
pub struct MutedWordsPref {
    /// A list of muted words.
    #[serde(rename = "items")]
    pub items: Vec<MutedWord>
}

/// Represents hidden posts preferences.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#hiddenPostsPref")]
pub struct HiddenPostsPref {
    /// A list of URIs of hidden posts by the user.
    #[serde(rename = "items")]
    pub items: Vec<String>
}

/// Represents labelers preferences.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#labelersPref")]
pub struct LabelersPref {
    /// A list of labeler preferences.
    #[serde(rename = "labelers")]
    pub labelers: Vec<LabelerPrefItem>
}

/// A labeler preference item.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "app.bsky.actor.defs#labelerPrefItem")]
pub struct LabelerPrefItem {
    /// The DID of the labeler.
    #[serde(rename = "did")]
    pub did: String
}
