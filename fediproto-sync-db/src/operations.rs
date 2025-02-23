use anyhow::Result;
use diesel::prelude::*;

use crate::error::FediProtoSyncDbError;

/// Get a synced Mastodon post by its ID from the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `mastodon_post_id` - The Mastodon post ID to get.
pub fn get_synced_mastodon_post_by_id(
    db_connection: &mut crate::AnyConnection,
    mastodon_post_id: &str
) -> Result<crate::models::MastodonPost, FediProtoSyncDbError> {
    let post = crate::schema::mastodon_posts::table
        .filter(crate::schema::mastodon_posts::post_id.eq(mastodon_post_id))
        .first::<crate::models::MastodonPost>(db_connection)
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

    Ok(post)
}

/// Check if a synced Mastodon post exists by its ID in the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `mastodon_post_id` - The Mastodon post ID to check.
pub fn check_synced_mastodon_post_exists(
    db_connection: &mut crate::AnyConnection,
    mastodon_post_id: &str
) -> bool {
    let post = get_synced_mastodon_post_by_id(db_connection, &mastodon_post_id);

    let post_exists = match post {
        Ok(_) => true,
        Err(_) => false
    };

    return post_exists;
}

/// Get the last synced Mastodon post ID from the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
pub fn get_last_synced_mastodon_post_id(
    db_connection: &mut crate::AnyConnection
) -> Result<Option<String>, FediProtoSyncDbError> {
    let last_synced_post_id = crate::schema::mastodon_posts::table
        .order(crate::schema::mastodon_posts::created_at.desc())
        .select(crate::schema::mastodon_posts::post_id)
        .first::<String>(db_connection)
        .optional()
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

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
) -> Result<(), FediProtoSyncDbError> {
    diesel::insert_into(crate::schema::mastodon_posts::table)
        .values(new_post)
        .execute(db_connection)
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

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
) -> Result<crate::models::SyncedPostBlueSkyData, FediProtoSyncDbError> {
    let synced_post = crate::schema::synced_posts_bluesky_data::table
        .filter(crate::schema::synced_posts_bluesky_data::mastodon_post_id.eq(mastodon_post_id))
        .first::<crate::models::SyncedPostBlueSkyData>(db_connection)
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

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
) -> Result<(), FediProtoSyncDbError> {
    diesel::insert_into(crate::schema::synced_posts_bluesky_data::table)
        .values(synced_post_data)
        .execute(db_connection)
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

    Ok(())
}

/// Get records of cached files from the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
pub fn get_cached_file_records(
    db_connection: &mut crate::AnyConnection
) -> Result<Vec<crate::models::CachedFile>, FediProtoSyncDbError> {
    let cached_files = crate::schema::cached_files::table
        .select(crate::models::CachedFile::as_select())
        .load(db_connection)
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

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
) -> Result<(), FediProtoSyncDbError> {
    diesel::insert_into(crate::schema::cached_files::table)
        .values(new_cached_file)
        .execute(db_connection)
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

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
) -> Result<(), FediProtoSyncDbError> {
    diesel::delete(crate::schema::cached_files::table)
        .filter(crate::schema::cached_files::id.eq(cached_file.id))
        .execute(db_connection)
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

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
) -> Result<Option<crate::models::CachedServiceToken>, FediProtoSyncDbError> {
    let token = crate::schema::cached_service_tokens::table
        .filter(crate::schema::cached_service_tokens::service_name.eq(service_name))
        .first::<crate::models::CachedServiceToken>(db_connection)
        .optional()
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

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
) -> Result<(), FediProtoSyncDbError> {
    diesel::insert_into(crate::schema::cached_service_tokens::table)
        .values(new_token)
        .execute(db_connection)
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

    Ok(())
}

/// Get Mastodon post retry queue items from the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
pub fn get_mastodon_post_retry_queue_items(
    db_connection: &mut crate::AnyConnection
) -> Result<Vec<crate::models::MastodonPostRetryQueueItem>, FediProtoSyncDbError> {
    let items = crate::schema::mastodon_post_retry_queue::table
        .select(crate::models::MastodonPostRetryQueueItem::as_select())
        .load(db_connection)
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

    Ok(items)
}

/// Get a Mastodon post retry queue item by the Mastodon post ID.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `mastodon_post_id` - The Mastodon post ID to get.
pub fn get_mastodon_post_retry_queue_item_by_post_id(
    db_connection: &mut crate::AnyConnection,
    mastodon_post_id: &i64
) -> Result<Option<crate::models::MastodonPostRetryQueueItem>, FediProtoSyncDbError> {
    let item = crate::schema::mastodon_post_retry_queue::table
        .filter(crate::schema::mastodon_post_retry_queue::id.eq(mastodon_post_id))
        .select(crate::models::MastodonPostRetryQueueItem::as_select())
        .first::<crate::models::MastodonPostRetryQueueItem>(db_connection)
        .optional()
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

    Ok(item)
}

/// Insert a new Mastodon post retry queue item into the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `new_item` - The new item to insert.
pub fn insert_mastodon_post_retry_queue_item(
    db_connection: &mut crate::AnyConnection,
    new_item: &crate::models::NewMastodonPostRetryQueueItem
) -> Result<(), FediProtoSyncDbError> {
    diesel::insert_into(crate::schema::mastodon_post_retry_queue::table)
        .values(new_item)
        .execute(db_connection)
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

    Ok(())
}

/// Update a Mastodon post retry queue item in the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `item` - The item to update.
pub fn update_mastodon_post_retry_queue_item(
    db_connection: &mut crate::AnyConnection,
    item: &crate::models::MastodonPostRetryQueueItem,
    new_reason: Option<&str>
) -> Result<(), FediProtoSyncDbError> {
    use crate::schema::mastodon_post_retry_queue::dsl::*;
    let updated_reason = match new_reason {
        Some(reason) => reason,
        None => item.failure_reason.as_str()
    };

    diesel::update(item)
        .set((
            failure_reason.eq(updated_reason),
            last_retried_at.eq(diesel::dsl::now),
            retry_count.eq(retry_count + 1)
        ))
        .execute(db_connection)
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

    Ok(())
}

/// Delete a Mastodon post retry queue item from the database.
///
/// ## Arguments
///
/// * `db_connection` - The database connection to use.
/// * `item` - The item to delete.
pub fn delete_mastodon_post_retry_queue_item(
    db_connection: &mut crate::AnyConnection,
    item: &crate::models::MastodonPostRetryQueueItem
) -> Result<(), FediProtoSyncDbError> {
    diesel::delete(crate::schema::mastodon_post_retry_queue::table)
        .filter(crate::schema::mastodon_post_retry_queue::id.eq(&item.id))
        .execute(db_connection)
        .map_err(|e| FediProtoSyncDbError::DatabaseOperationError(e))?;

    Ok(())
}
