use diesel::{
    sqlite::Sqlite, Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SqliteConnection
};

use crate::{bsky, mastodon::MastodonApiExtensions, models, schema, FediProtoSyncEnvVars};

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations = diesel_migrations::embed_migrations!("./migrations");

pub async fn run(config: FediProtoSyncEnvVars) -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let db_connection = &mut SqliteConnection::establish(&database_url)?;
    tracing::info!("Connected to database.");

    run_migrations(db_connection).expect("Failed to run migrations.");

    let mut interval = tokio::time::interval(config.sync_interval);

    loop {
        interval.tick().await;

        tracing::info!("Running sync...");
        let sync_result = run_sync(config.clone(), db_connection).await;

        match sync_result {
            Ok(_) => {
                tracing::info!("Sync completed successfully.");
            }
            Err(e) => {
                tracing::error!("Sync failed: {:#?}", e);
            }
        }
    }
}

fn run_migrations(
    connection: &mut impl diesel_migrations::MigrationHarness<Sqlite>
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let pending_migrations = connection.pending_migrations(MIGRATIONS)?;

    if pending_migrations.is_empty() {
        tracing::info!("No pending database migrations.");
        return Ok(());
    }

    tracing::info!("Applying '{}' pending database migrations...", pending_migrations.len());

    for migration_item in pending_migrations {
        connection.run_migration(&migration_item)?;
        tracing::info!("Applied migration '{}'", migration_item.name());
    }

    tracing::info!("Applied all pending database migrations.");

    Ok(())
}

async fn run_sync(
    config: FediProtoSyncEnvVars,
    db_connection: &mut SqliteConnection
) -> Result<(), Box<dyn std::error::Error>> {
    let mastodon_client = megalodon::generator(
        megalodon::SNS::Mastodon,
        format!("https://{}", config.mastodon_server.clone()),
        Some(config.mastodon_access_token.clone()),
        None
    )?;
    let account = mastodon_client.verify_account_credentials().await?;
    tracing::info!("Authenticated to Mastodon as '{}'", account.json.username);

    let bsky_auth = bsky::create_session_token(&config).await?;
    tracing::info!("Authenticated to BlueSky as '{}'", bsky_auth.session.handle);

    let last_synced_post_id = schema::mastodon_posts::table
        .order(schema::mastodon_posts::created_at.desc())
        .select(schema::mastodon_posts::post_id)
        .first::<String>(db_connection)
        .optional()?;

    let mut latest_posts = mastodon_client
        .get_latest_posts(&account.json.id, last_synced_post_id.clone())
        .await?;

    latest_posts.json.reverse();

    if last_synced_post_id.clone().is_none() && latest_posts.json.len() > 0 {
        let initial_post = latest_posts.json[0].clone();

        diesel::insert_into(schema::mastodon_posts::table)
            .values(models::NewMastodonPost::new(&initial_post, None, None))
            .execute(db_connection)?;

        tracing::info!("Added initial post to database for future syncs.");

        return Ok(());
    }

    tracing::info!(
        "Retrieved '{}' new posts from Mastodon.",
        latest_posts.json.len()
    );

    for post_item in latest_posts.json {
        tracing::info!("Processing post '{}'", post_item.id);
        bsky::sync_post(&bsky_auth, db_connection, &account.json, post_item).await?;
    }

    Ok(())
}
