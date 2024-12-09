use diesel::prelude::*;

/// Get a synced Mastodon post by its ID from the database.
/// 
/// ## Arguments
/// 
/// * `db_connection` - The database connection to use.
/// * `mastodon_post_id` - The Mastodon post ID to get.
pub fn get_synced_mastodon_post_by_id(
    db_connection: &mut crate::db::AnyConnection,
    mastodon_post_id: &str
) -> Result<crate::db::models::MastodonPost, crate::error::Error> {
    let post = crate::schema::mastodon_posts::table
        .filter(crate::schema::mastodon_posts::post_id.eq(mastodon_post_id))
        .first::<crate::db::models::MastodonPost>(db_connection)
        .map_err(|e| {
            crate::error::Error::with_source(
                "Failed to get Mastodon post by ID.",
                crate::error::ErrorKind::DatabaseQueryError,
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
    db_connection: &mut crate::db::AnyConnection
) -> Result<Option<String>, crate::error::Error> {
    let last_synced_post_id = crate::schema::mastodon_posts::table
        .order(crate::schema::mastodon_posts::created_at.desc())
        .select(crate::schema::mastodon_posts::post_id)
        .first::<String>(db_connection)
        .optional()
        .map_err(|e| {
            crate::error::Error::with_source(
                "Failed to get the last synced Mastodon post ID.",
                crate::error::ErrorKind::DatabaseQueryError,
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
    db_connection: &mut crate::db::AnyConnection,
    new_post: &crate::db::models::NewMastodonPost
) -> Result<(), crate::error::Error> {
    diesel::insert_into(crate::schema::mastodon_posts::table)
        .values(new_post)
        .execute(db_connection)
        .map_err(|e| {
            crate::error::Error::with_source(
                "Failed to insert new synced Mastodon post.",
                crate::error::ErrorKind::DatabaseInsertError,
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
    db_connection: &mut crate::db::AnyConnection,
    mastodon_post_id: &str
) -> Result<crate::db::models::SyncedPostBlueSkyData, crate::error::Error> {
    let synced_post = crate::schema::synced_posts::table
        .filter(crate::schema::synced_posts::mastodon_post_id.eq(mastodon_post_id))
        .first::<crate::db::models::SyncedPostBlueSkyData>(db_connection)
        .map_err(|e| {
            crate::error::Error::with_source(
                "Failed to get synced post by post ID.",
                crate::error::ErrorKind::DatabaseQueryError,
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
    db_connection: &mut crate::db::AnyConnection,
    synced_post_data: &crate::db::models::NewSyncedPostBlueSkyData
) -> Result<(), crate::error::Error> {
    diesel::insert_into(crate::schema::synced_posts::table)
        .values(synced_post_data)
        .execute(db_connection)
        .map_err(|e| {
            crate::error::Error::with_source(
                "Failed to insert new synced post.",
                crate::error::ErrorKind::DatabaseInsertError,
                Box::new(e)
            )
        })?;

    Ok(())
}
