use chrono::NaiveDateTime;
use diesel::prelude::*;
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

/// Represents a synced post in the `synced_posts_bluesky_data` table.
#[derive(Queryable, Selectable, Clone, PartialEq, Debug)]
#[allow(dead_code)]
#[diesel(table_name = crate::schema::synced_posts_bluesky_data)]
pub struct SyncedPostBlueSkyData {
    /// A unique identifier for the synced post in the database.
    pub id: crate::db::type_impls::UuidProxy,

    /// The Mastodon post ID.
    pub mastodon_post_id: String,

    /// The CID of the BlueSky post.
    pub bsky_post_cid: String,

    /// The URI of the BlueSky post.
    pub bsky_post_uri: String
}

/// Represents a new synced post to insert into the `synced_posts_bluesky_data` table.
#[derive(Insertable)]
#[diesel(table_name = crate::schema::synced_posts_bluesky_data)]
pub struct NewSyncedPostBlueSkyData {
    /// A unique identifier for the synced post in the database.
    pub id: crate::db::type_impls::UuidProxy,

    /// The Mastodon post ID.
    pub mastodon_post_id: String,

    /// The CID of the BlueSky post.
    pub bsky_post_cid: String,

    /// The URI of the BlueSky post.
    pub bsky_post_uri: String
}

impl NewSyncedPostBlueSkyData {
    /// Create a new instance of the `NewSyncedPostBlueSkyData` struct.
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
    /// Remove a cached file from the database and the file system.
    ///
    /// ## Arguments
    ///
    /// * `db_connection` - The database connection to use.
    pub async fn remove_file(
        &self,
        db_connection: &mut crate::db::AnyConnection
    ) -> Result<(), crate::error::Error> {
        crate::db::operations::delete_cached_file_record(db_connection, self)?;

        let file_path = std::path::Path::new(&self.file_path);

        if file_path.exists() {
            tokio::fs::remove_file(&file_path).await.map_err(|e| {
                crate::error::Error::with_source(
                    "Failed to remove cached file.",
                    crate::error::ErrorKind::TempFileRemovalError,
                    Box::new(e)
                )
            })?;
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

/// Represents a cached service token in the `cached_service_tokens` table.
#[derive(Queryable, Selectable, Clone, PartialEq, Debug)]
#[allow(dead_code)]
#[diesel(table_name = crate::schema::cached_service_tokens)]
pub struct CachedServiceToken {
    /// A unique identifier for the cached service token in the database.
    pub id: crate::db::type_impls::UuidProxy,

    /// The name of the service the token is for.
    pub service_name: String,

    /// The encrypted access token for the service.
    pub access_token: String,

    /// The encrypted refresh token for the service, if any.
    pub refresh_token: Option<String>,

    /// The time in seconds until the access token expires, if any.
    pub expires_in: Option<i32>,

    /// The scopes the access token has, if any.
    pub scopes: Option<String>
}

/// Trait for decrypting a cached service token's access and refresh tokens.
pub trait CachedServiceTokenDecrypt {
    /// Decrypt the access token.
    /// 
    /// ## Arguments
    /// 
    /// * `encryption_private_key` - The private key to use for decryption.
     fn decrypt_access_token(
        &self,
        encryption_private_key: &boring::rsa::Rsa<boring::pkey::Private>
    ) -> Result<String, crate::error::Error>;

    /// Decrypt the refresh token.
    /// 
    /// ## Arguments
    /// 
    /// * `encryption_private_key` - The private key to use for decryption.
    /// 
    /// ## Note
    /// 
    /// If there is no refresh token, this method will return `None`.
    #[allow(dead_code)]
     fn decrypt_refresh_token(
        &self,
        encryption_private_key: &boring::rsa::Rsa<boring::pkey::Private>
    ) -> Result<Option<String>, crate::error::Error>;
}

impl CachedServiceTokenDecrypt for CachedServiceToken {
    /// Decrypt the access token.
    /// 
    /// ## Arguments
    /// 
    /// * `encryption_private_key` - The private key to use for decryption.
     fn decrypt_access_token(
        &self,
        encryption_private_key: &boring::rsa::Rsa<boring::pkey::Private>
    ) -> Result<String, crate::error::Error> {
        let decrypted_access_token = crate::crypto::decrypt_string(encryption_private_key, &self.access_token)?;

        Ok(decrypted_access_token)
    }

    /// Decrypt the refresh token.
    /// 
    /// ## Arguments
    /// 
    /// * `encryption_private_key` - The private key to use for decryption.
    ///
    /// ## Note
    /// 
    /// If there is no refresh token, this method will return `None`.
    #[allow(dead_code)]
     fn decrypt_refresh_token(
        &self,
        encryption_private_key: &boring::rsa::Rsa<boring::pkey::Private>
    ) -> Result<Option<String>, crate::error::Error> {
        let decrypted_refresh_token = match &self.refresh_token {
            Some(refresh_token) => {
                let decrypted_refresh_token = crate::crypto::decrypt_string(encryption_private_key, refresh_token)?;

                Some(decrypted_refresh_token)
            }

            None => None
        };

        Ok(decrypted_refresh_token)
    }
}

/// Represents a new cached service token to insert into the `cached_service_tokens` table.
#[derive(Insertable)]
#[diesel(table_name = crate::schema::cached_service_tokens)]
pub struct NewCachedServiceToken {
    /// A unique identifier for the cached service token in the database.
    pub id: crate::db::type_impls::UuidProxy,

    /// The name of the service the token is for.
    pub service_name: String,

    /// The encrypted access token for the service.
    pub access_token: String,

    /// The encrypted refresh token for the service, if any.
    pub refresh_token: Option<String>,

    /// The time in seconds until the access token expires, if any.
    pub expires_in: Option<i32>,

    /// The scopes the access token has, if any.
    pub scopes: Option<String>
}

impl NewCachedServiceToken {
    /// Create a new instance of the `NewCachedServiceToken` struct.
    /// 
    /// ## Arguments
    /// 
    /// * `encryption_public_key` - The public key to use for encryption.
    /// * `service_name` - The name of the service the token is for.
    /// * `access_token` - The access token to encrypt.
    /// * `refresh_token` - The refresh token to encrypt, if any.
    /// * `expires_in` - The time in seconds until the access token expires, if any.
    /// * `scopes` - The scopes the access token has, if any.
    pub fn new(
        encryption_public_key: &boring::rsa::Rsa<boring::pkey::Public>,
        service_name: &str,
        access_token: &str,
        refresh_token: Option<String>,
        expires_in: Option<i32>,
        scopes: Option<String>
    ) -> Result<Self, crate::error::Error> {
        let time_context = uuid::ContextV7::new();
        let id = uuid::Uuid::new_v7(uuid::Timestamp::now(&time_context));

        let service_name = service_name.to_string();

        let encrypted_access_token = crate::crypto::encrypt_string(encryption_public_key, access_token)?;

        let encrypted_refresh_token = match refresh_token {
            Some(refresh_token) => {
                let encrypted_refresh_token = crate::crypto::encrypt_string(encryption_public_key, &refresh_token)?;

                Some(encrypted_refresh_token)
            }

            None => None
        };

        Ok(Self {
            id: UuidProxy(id),
            service_name,
            access_token: encrypted_access_token,
            refresh_token: encrypted_refresh_token,
            expires_in,
            scopes
        })
    }
}
