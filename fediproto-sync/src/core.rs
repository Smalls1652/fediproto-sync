use diesel::{
    sqlite::Sqlite,
    Connection,
    ExpressionMethods,
    OptionalExtension,
    QueryDsl,
    RunQueryDsl,
    SqliteConnection
};

use crate::{bsky, mastodon::MastodonApiExtensions, models, schema, FediProtoSyncEnvVars};

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("./migrations");

struct AuthSessions {
    bsky: bsky::BlueSkyAuthentication
}

impl AuthSessions {
    pub fn new(bsky: bsky::BlueSkyAuthentication) -> Self {
        Self { bsky }
    }

    pub async fn authenticate(config: &FediProtoSyncEnvVars) -> Result<Self, crate::error::Error> {
        // Authenticate to BlueSky.
        let bsky_auth = bsky::create_session_token(&config).await?;
        tracing::info!("Authenticated to BlueSky as '{}'", bsky_auth.session.handle);

        Ok(Self::new(bsky_auth))
    }

    pub async fn refresh_bsky_auth(&mut self) -> Result<(), crate::error::Error> {
        self.bsky = bsky::refresh_session_token(&self.bsky).await?;

        tracing::info!("Refreshed BlueSky authentication token.");

        Ok(())
    }
}

/// Run the main sync loop.
///
/// This function will run the main sync loop for the FediProtoSync application.
/// It will connect to the database, run any pending migrations, and then run
/// the sync loop at the specified interval.
///
/// ## Arguments
///
/// - `config` - The environment variables for the FediProtoSync application.
pub async fn run(config: FediProtoSyncEnvVars) -> Result<(), crate::error::Error> {
    // TODO: Should probably add this to the `FediProtoSyncEnvVars` struct.
    let database_url = std::env::var("DATABASE_URL").map_err(|e| {
        crate::error::Error::with_source(
            "Failed to read DATABASE_URL environment variable.",
            crate::error::ErrorKind::EnvironmentVariableError,
            Box::new(e)
        )
    })?;

    let db_connection = &mut SqliteConnection::establish(&database_url).map_err(|e| {
        crate::error::Error::with_source(
            "Failed to connect to database.",
            crate::error::ErrorKind::DatabaseConnectionError,
            Box::new(e)
        )
    })?;
    tracing::info!("Connected to database.");

    run_migrations(db_connection).expect("Failed to run migrations.");

    let mut auth_sessions = AuthSessions::authenticate(&config).await.map_err(|e| {
        crate::error::Error::with_source(
            "Failed to authenticate to BlueSky.",
            crate::error::ErrorKind::AuthenticationError,
            Box::new(e)
        )
    })?;

    // Run the sync loop.
    let mut interval = tokio::time::interval(config.sync_interval);
    loop {
        interval.tick().await;

        tracing::info!("Running sync...");

        let sync_result = run_sync(&config, db_connection, &mut auth_sessions).await;

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

/// Run any pending database migrations.
///
/// ## Arguments
///
/// - `connection` - The database connection to run the migrations on.
fn run_migrations(
    connection: &mut impl diesel_migrations::MigrationHarness<Sqlite>
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let pending_migrations = connection.pending_migrations(MIGRATIONS)?;

    if pending_migrations.is_empty() {
        tracing::info!("No pending database migrations.");
        return Ok(());
    }

    tracing::info!(
        "Applying '{}' pending database migrations...",
        pending_migrations.len()
    );

    for migration_item in pending_migrations {
        connection.run_migration(&migration_item)?;
        tracing::info!("Applied migration '{}'", migration_item.name());
    }

    tracing::info!("Applied all pending database migrations.");

    Ok(())
}

/// Run the Mastodon to BlueSky sync.
///
/// ## Arguments
///
/// - `config` - The environment variables for the FediProtoSync application.
/// - `db_connection` - The database connection to use for the sync.
async fn run_sync(
    config: &FediProtoSyncEnvVars,
    db_connection: &mut SqliteConnection,
    auth_sessions: &mut AuthSessions
) -> Result<(), Box<dyn std::error::Error>> {
    // Create the Mastodon client and authenticate.
    let mastodon_client = megalodon::generator(
        megalodon::SNS::Mastodon,
        format!("https://{}", config.mastodon_server.clone()),
        Some(config.mastodon_access_token.clone()),
        None
    )
    .map_err(|e| {
        crate::error::Error::with_source(
            "Failed to create Mastodon client.",
            crate::error::ErrorKind::AuthenticationError,
            Box::new(e)
        )
    })?;

    let account = mastodon_client
        .verify_account_credentials()
        .await
        .map_err(|e| {
            crate::error::Error::with_source(
                "Failed to verify Mastodon account credentials.",
                crate::error::ErrorKind::AuthenticationError,
                Box::new(e)
            )
        })?;
    tracing::info!("Authenticated to Mastodon as '{}'", account.json.username);

    auth_sessions.refresh_bsky_auth().await?;

    // Get the last synced post ID, if any.
    let last_synced_post_id = schema::mastodon_posts::table
        .order(schema::mastodon_posts::created_at.desc())
        .select(schema::mastodon_posts::post_id)
        .first::<String>(db_connection)
        .optional()?;

    // Get the latest posts from Mastodon.
    // If there is no last synced post ID, we will only get the latest post.
    // Otherwise, we will get all posts since the last synced post.
    let mut latest_posts = mastodon_client
        .get_latest_posts(&account.json.id, last_synced_post_id.clone())
        .await?;

    // Reverse the posts so we process them in ascending order.
    latest_posts.json.reverse();

    // If there is no last synced post ID, we need to add the initial post to the
    // database. This is so we have a starting point for future syncs.
    //
    // Note: The initial post **is not synced** to BlueSky.
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

    // Process each new post and sync it to BlueSky.
    for post_item in latest_posts.json {
        tracing::info!("Processing post '{}'", post_item.id);

        let sync_result = bsky::sync_post(
            &config,
            &auth_sessions.bsky,
            db_connection,
            &account.json,
            &post_item
        )
        .await;

        match sync_result {
            Ok(_) => {
                tracing::info!("Post '{}' processed successfully.", post_item.id);
            }
            Err(e) => {
                tracing::error!("Failed to process post '{}': {:#?}", post_item.id, e);
            }
        }
    }

    Ok(())
}
