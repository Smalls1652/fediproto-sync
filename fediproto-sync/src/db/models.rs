use chrono::NaiveDateTime;
use diesel::{
    prelude::*,
    ExpressionMethods,
    RunQueryDsl
};
use megalodon::entities::Status;

use super::type_impls::UuidProxy;

/// Represents a Mastodon post in the `mastodon_posts` table.
#[derive(Queryable, Selectable, PartialEq, Debug)]
#[allow(dead_code)]
#[diesel(table_name = crate::schema::mastodon_posts)]
pub struct MastodonPost {
    /// A unique identifier for the Mastodon post in the database.
    pub id: crate::db::type_impls::UuidProxy,

    /// The Mastodon account ID that created the post.
    pub account_id: String,

    /// The Mastodon post ID.
    pub post_id: String,

    /// The date and time the post was created.
    pub created_at: NaiveDateTime,

    /// Whether the post is a thread post.
    pub is_thread_post: bool,

    /// The previous post ID in the thread, if any.
    pub previous_post_id: Option<String>,

    /// The BlueSky post ID when the post was synced, if any.
    pub bsky_post_id: Option<String>,

    /// The root Mastodon post ID in the thread, if any.
    pub root_mastodon_post_id: Option<String>
}

/// Represents a new Mastodon post to insert into the `mastodon_posts` table.
#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::mastodon_posts)]
pub struct NewMastodonPost {
    /// A unique identifier for the Mastodon post in the database.
    pub id: crate::db::type_impls::UuidProxy,

    /// The Mastodon account ID that created the post.
    pub account_id: String,

    /// The Mastodon post ID.
    pub post_id: String,

    /// The date and time the post was created.
    pub created_at: NaiveDateTime,

    /// Whether the post is a thread post.
    pub is_thread_post: bool,

    /// The previous post ID in the thread, if any.
    pub previous_post_id: Option<String>,

    /// The BlueSky post ID when the post was synced, if any.
    pub bsky_post_id: Option<String>,

    /// The root Mastodon post ID in the thread, if any.
    pub root_mastodon_post_id: Option<String>
}

impl NewMastodonPost {
    /// Create a new instance of the `NewMastodonPost` struct.
    ///
    /// ## Arguments
    ///
    /// * `post` - The Mastodon post to create a new post from.
    /// * `bsky_post_id` - The BlueSky post ID when the post was synced, if any.
    /// * `root_mastodon_post_id` - The root Mastodon post ID in the thread, if
    ///   any.
    pub fn new(
        post: &Status,
        bsky_post_id: Option<String>,
        root_mastodon_post_id: Option<String>
    ) -> Self {
        let time_context = uuid::ContextV7::new();
        let id = uuid::Uuid::new_v7(uuid::Timestamp::now(&time_context));

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
            id: UuidProxy(id),
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

/// Represents a synced post in the `synced_posts` table.
#[derive(Queryable, Selectable, Clone, PartialEq, Debug)]
#[allow(dead_code)]
#[diesel(table_name = crate::schema::synced_posts)]
pub struct SyncedPost {
    /// A unique identifier for the synced post in the database.
    pub id: crate::db::type_impls::UuidProxy,

    /// The Mastodon post ID.
    pub mastodon_post_id: String,

    /// The CID of the BlueSky post.
    pub bsky_post_cid: String,

    /// The URI of the BlueSky post.
    pub bsky_post_uri: String
}

/// Represents a new synced post to insert into the `synced_posts` table.
#[derive(Insertable)]
#[diesel(table_name = crate::schema::synced_posts)]
pub struct NewSyncedPost {
    /// A unique identifier for the synced post in the database.
    pub id: crate::db::type_impls::UuidProxy,

    /// The Mastodon post ID.
    pub mastodon_post_id: String,

    /// The CID of the BlueSky post.
    pub bsky_post_cid: String,

    /// The URI of the BlueSky post.
    pub bsky_post_uri: String
}

impl NewSyncedPost {
    /// Create a new instance of the `NewSyncedPost` struct.
    ///
    /// ## Arguments
    ///
    /// * `mastodon_post_id` - The Mastodon post ID.
    /// * `bsky_post_cid` - The CID of the BlueSky post.
    /// * `bsky_post_uri` - The URI of the BlueSky post.
    pub fn new(
        mastodon_post_id: &str,
        bsky_post_cid: &str,
        bsky_post_uri: &str
    ) -> Self {
        let time_context = uuid::ContextV7::new();
        let id = uuid::Uuid::new_v7(uuid::Timestamp::now(&time_context));

        Self {
            id: UuidProxy(id),
            mastodon_post_id: mastodon_post_id.to_string(),
            bsky_post_cid: bsky_post_cid.to_string(),
            bsky_post_uri: bsky_post_uri.to_string()
        }
    }
}

/// Represents a cached file in the `cached_files` table.
#[derive(Queryable, Selectable, PartialEq, Debug)]
#[allow(dead_code)]
#[diesel(table_name = crate::schema::cached_files)]
pub struct CachedFile {
    /// A unique identifier for the cached file in the database.
    pub id: crate::db::type_impls::UuidProxy,

    /// The path to the cached file.
    pub file_path: String
}

impl CachedFile {
    /// Remove the cached file from the file system.
    pub async fn remove_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = std::path::Path::new(&self.file_path);

        if file_path.exists() {
            tokio::fs::remove_file(&file_path).await?;
        }

        Ok(())
    }
}

/// Represents a new cached file to insert into the `cached_files` table.
#[derive(Insertable)]
#[diesel(table_name = crate::schema::cached_files)]
pub struct NewCachedFile {
    /// A unique identifier for the cached file in the database.
    pub id: crate::db::type_impls::UuidProxy,

    /// The path to the cached file.
    pub file_path: String
}

impl NewCachedFile {
    /// Create a new instance of the `NewCachedFile` struct.
    /// 
    /// ## Arguments
    /// 
    /// * `file_path` - The path to the cached file.
    pub fn new(file_path: &std::path::PathBuf) -> Self {
        let time_context = uuid::ContextV7::new();
        let id = uuid::Uuid::new_v7(uuid::Timestamp::now(&time_context));

        Self {
            id: UuidProxy(id),
            file_path: file_path.to_string_lossy().to_string()
        }
    }
}

/// Remove a cached file from the database and the file system.
///
/// ## Arguments
///
/// * `cached_file` - The cached file to remove.
/// * `db_connection` - The database connection to use.
pub async fn remove_cached_file(
    cached_file: &CachedFile,
    db_connection: &mut crate::db::AnyConnection
) -> Result<(), Box<dyn std::error::Error>> {
    diesel::delete(crate::schema::cached_files::table)
        .filter(crate::schema::cached_files::id.eq(cached_file.id))
        .execute(db_connection)?;

    cached_file.remove_file().await?;

    Ok(())
}
