use diesel::prelude::*;
use fediproto_sync_lib::error::{FediProtoSyncError, FediProtoSyncErrorKind};

/// Get a synced Mastodon post by its ID from the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `mastodon_post_id` - The Mastodon post ID to get.
pub fn get_synced_mastodon_post_by_id(
    db_connection: &mut crate::AnyConnection,
    mastodon_post_id: &str
) -> Result<crate::models::MastodonPost, FediProtoSyncError> {
    let post = crate::schema::mastodon_posts::table
        .filter(crate::schema::mastodon_posts::post_id.eq(mastodon_post_id))
        .first::<crate::models::MastodonPost>(db_connection)
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to get Mastodon post by ID.",
                FediProtoSyncErrorKind::DatabaseQueryError,
                Box::new(e)
            )
        })?;

    Ok(post)
}

/// Get the last synced Mastodon post ID from the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
pub fn get_last_synced_mastodon_post_id(
    db_connection: &mut crate::AnyConnection
) -> Result<Option<String>, FediProtoSyncError> {
    let last_synced_post_id = crate::schema::mastodon_posts::table
        .order(crate::schema::mastodon_posts::created_at.desc())
        .select(crate::schema::mastodon_posts::post_id)
        .first::<String>(db_connection)
        .optional()
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to get the last synced Mastodon post ID.",
                FediProtoSyncErrorKind::DatabaseQueryError,
                Box::new(e)
            )
        })?;

    Ok(last_synced_post_id)
}

/// Insert a new synced Mastodon post into the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `new_post` - The new post to insert.
pub fn insert_new_synced_mastodon_post(
    db_connection: &mut crate::AnyConnection,
    new_post: &crate::models::NewMastodonPost
) -> Result<(), FediProtoSyncError> {
    diesel::insert_into(crate::schema::mastodon_posts::table)
        .values(new_post)
        .execute(db_connection)
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to insert new synced Mastodon post.",
                FediProtoSyncErrorKind::DatabaseInsertError,
                Box::new(e)
            )
        })?;

    Ok(())
}

/// Get BlueSky data of a synced Mastodon post by its Mastodon post ID.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `mastodon_post_id` - The Mastodon post ID to get.
pub fn get_bluesky_data_by_mastodon_post_id(
    db_connection: &mut crate::AnyConnection,
    mastodon_post_id: &str
) -> Result<crate::models::SyncedPostBlueSkyData, FediProtoSyncError> {
    let synced_post = crate::schema::synced_posts_bluesky_data::table
        .filter(crate::schema::synced_posts_bluesky_data::mastodon_post_id.eq(mastodon_post_id))
        .first::<crate::models::SyncedPostBlueSkyData>(db_connection)
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to get synced post by post ID.",
                FediProtoSyncErrorKind::DatabaseQueryError,
                Box::new(e)
            )
        })?;

    Ok(synced_post)
}

/// Insert new BlueSky data for a synced Mastodon post into the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `synced_post_data` - The new synced post to insert.
pub fn insert_new_bluesky_data_for_synced_mastodon_post(
    db_connection: &mut crate::AnyConnection,
    synced_post_data: &crate::models::NewSyncedPostBlueSkyData
) -> Result<(), FediProtoSyncError> {
    diesel::insert_into(crate::schema::synced_posts_bluesky_data::table)
        .values(synced_post_data)
        .execute(db_connection)
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to insert new synced post.",
                FediProtoSyncErrorKind::DatabaseInsertError,
                Box::new(e)
            )
        })?;

    Ok(())
}

/// Get records of cached files from the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
pub fn get_cached_file_records(
    db_connection: &mut crate::AnyConnection
) -> Result<Vec<crate::models::CachedFile>, FediProtoSyncError> {
    let cached_files = crate::schema::cached_files::table
        .select(crate::models::CachedFile::as_select())
        .load(db_connection)
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to get cached files.",
                FediProtoSyncErrorKind::DatabaseQueryError,
                Box::new(e)
            )
        })?;

    Ok(cached_files)
}

/// Insert a new cached file record into the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `new_cached_file` - The new cached file record to insert.
pub fn insert_cached_file_record(
    db_connection: &mut crate::AnyConnection,
    new_cached_file: &crate::models::NewCachedFile
) -> Result<(), FediProtoSyncError> {
    diesel::insert_into(crate::schema::cached_files::table)
        .values(new_cached_file)
        .execute(db_connection)
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to insert new cached file.",
                FediProtoSyncErrorKind::DatabaseInsertError,
                Box::new(e)
            )
        })?;

    Ok(())
}

/// Delete a cached file record from the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `cached_file` - The cached file record to delete.
pub fn delete_cached_file_record(
    db_connection: &mut crate::AnyConnection,
    cached_file: &crate::models::CachedFile
) -> Result<(), FediProtoSyncError> {
    diesel::delete(crate::schema::cached_files::table)
        .filter(crate::schema::cached_files::id.eq(cached_file.id))
        .execute(db_connection)
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to delete cached file record.",
                FediProtoSyncErrorKind::DatabaseDeleteError,
                Box::new(e)
            )
        })?;

    Ok(())
}

/// Get a cached service token by its service name from the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `service_name` - The service name to get.
pub fn get_cached_service_token_by_service_name(
    db_connection: &mut crate::AnyConnection,
    service_name: &str
) -> Result<Option<crate::models::CachedServiceToken>, FediProtoSyncError> {
    let token = crate::schema::cached_service_tokens::table
        .filter(crate::schema::cached_service_tokens::service_name.eq(service_name))
        .first::<crate::models::CachedServiceToken>(db_connection)
        .optional()
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to get cached service token by service name.",
                FediProtoSyncErrorKind::DatabaseQueryError,
                Box::new(e)
            )
        })?;

    Ok(token)
}

/// Insert a new cached service token into the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `new_token` - The new token to insert.
pub fn insert_cached_service_token(
    db_connection: &mut crate::AnyConnection,
    new_token: &crate::models::NewCachedServiceToken
) -> Result<(), FediProtoSyncError> {
    diesel::insert_into(crate::schema::cached_service_tokens::table)
        .values(new_token)
        .execute(db_connection)
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to insert new cached service token.",
                FediProtoSyncErrorKind::DatabaseInsertError,
                Box::new(e)
            )
        })?;

    Ok(())
}
