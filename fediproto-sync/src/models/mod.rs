use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable, Selectable};
use megalodon::entities::Status;

#[derive(Queryable, Selectable)]
#[allow(dead_code)]
#[diesel(table_name = crate::schema::mastodon_posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MastodonPost {
    pub id: i32,
    pub account_id: String,
    pub post_id: String,
    pub created_at: NaiveDateTime,
    pub is_thread_post: bool,
    pub previous_post_id: Option<String>,
    pub bsky_post_id: Option<String>,
    pub root_mastodon_post_id: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::mastodon_posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewMastodonPost {
    pub account_id: String,
    pub post_id: String,
    pub created_at: NaiveDateTime,
    pub is_thread_post: bool,
    pub previous_post_id: Option<String>,
    pub bsky_post_id: Option<String>,
    pub root_mastodon_post_id: Option<String>
}

impl NewMastodonPost {
    pub fn new(
        post: &Status,
        bsky_post_id: Option<String>,
        root_mastodon_post_id: Option<String>
    ) -> Self {
        let account_id = post.account.id.clone();
        let post_id = post.id.clone();
        let created_at = post.created_at.clone().naive_utc();

        let post_in_reply_to_id = post.in_reply_to_id.clone();

        let is_reply = post_in_reply_to_id.is_some();

        let is_thread_post = match is_reply {
            true => {
                let post_in_reply_to_account_id = post.in_reply_to_account_id.clone().unwrap();

                post_in_reply_to_account_id == account_id
            }

            false => false
        };

        let previous_post_id = match is_thread_post {
            true => Some(post_in_reply_to_id.unwrap()),
            false => None
        };

        Self {
            account_id,
            post_id,
            created_at,
            is_thread_post,
            previous_post_id,
            bsky_post_id,
            root_mastodon_post_id
        }
    }
}

#[derive(Queryable, Selectable, Clone)]
#[allow(dead_code)]
#[diesel(table_name = crate::schema::synced_posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SyncedPost {
    pub id: i32,
    pub mastodon_post_id: String,
    pub bsky_post_cid: String,
    pub bsky_post_uri: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::synced_posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewSyncedPost {
    pub mastodon_post_id: String,
    pub bsky_post_cid: String,
    pub bsky_post_uri: String
}

impl NewSyncedPost {
    pub fn new(
        mastodon_post_id: &str,
        bsky_post_cid: &str,
        bsky_post_uri: &str
    ) -> Self {
        Self {
            mastodon_post_id: mastodon_post_id.to_string(),
            bsky_post_cid: bsky_post_cid.to_string(),
            bsky_post_uri: bsky_post_uri.to_string()
        }
    }
}
