use atprotolib_rs::types::app_bsky;
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper
};

use crate::{bsky, mastodon::MastodonApiExtensions, models, schema, FediProtoSyncEnvVars};

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("./migrations");

/// The main sync loop for the FediProto Sync application.
pub struct FediProtoSyncLoop {
    /// The environment variables for the FediProto Sync application.
    config: FediProtoSyncEnvVars,

    /// The database connection for the FediProto Sync application.
    db_connection: PgConnection,

    /// The BlueSky authentication session.
    bsky_auth: bsky::BlueSkyAuthentication
}

impl FediProtoSyncLoop {
    /// Create a new instance of the FediProtoSyncLoop.
    ///
    /// ## Arguments
    ///
    /// * `config` - The environment variables for the FediProtoSync
    ///   application.
    pub async fn new(config: &FediProtoSyncEnvVars) -> Result<Self, Box<dyn std::error::Error>> {
        let config = config.clone();

        let database_url = config.database_url.clone();
        let db_connection = PgConnection::establish(&database_url).map_err(|e| {
            crate::error::Error::with_source(
                "Failed to connect to database.",
                crate::error::ErrorKind::DatabaseConnectionError,
                Box::new(e)
            )
        })?;
        tracing::info!("Connected to database.");

        let client = create_http_client(&config)?;
        let bsky_auth = bsky::BlueSkyAuthentication::new(&config, client).await?;
        tracing::info!(
            "Authenticated to BlueSky as '{}'.",
            &bsky_auth.session.handle
        );

        Ok(Self {
            config,
            db_connection,
            bsky_auth
        })
    }

    /// Run the main sync loop.
    ///
    /// This function will run the main sync loop for the FediProtoSync
    /// application. It will connect to the database, run any pending
    /// migrations, and then run the sync loop at the specified interval.
    ///
    /// ## Arguments
    ///
    /// * `config` - The environment variables for the FediProtoSync
    ///   application.
    pub async fn run_loop(&mut self) -> Result<(), crate::error::Error> {
        FediProtoSyncLoop::run_migrations(&mut self.db_connection)
            .expect("Failed to run migrations.");

        // Run the sync loop.
        let mut interval = tokio::time::interval(self.config.sync_interval);
        loop {
            interval.tick().await;

            tracing::info!("Running sync...");

            let sync_result = self.start_sync().await;

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
    /// * `connection` - The database connection to run the migrations on.
    fn run_migrations(
        connection: &mut impl diesel_migrations::MigrationHarness<diesel::pg::Pg>
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
    /// * `config` - The environment variables for the FediProtoSync
    ///   application.
    /// * `db_connection` - The database connection to use for the sync.
    async fn start_sync(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create the Mastodon client and authenticate.
        let mastodon_client = megalodon::generator(
            megalodon::SNS::Mastodon,
            format!("https://{}", self.config.mastodon_server.clone()),
            Some(self.config.mastodon_access_token.clone()),
            Some(self.config.user_agent.clone())
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

        let client = create_http_client(&self.config)?;
        self.bsky_auth.refresh_session_token(client).await?;
        tracing::info!("Refreshed BlueSky session token.");

        // Get the last synced post ID, if any.
        tracing::info!("Getting last synced post...");
        let last_synced_post_id = schema::mastodon_posts::table
            .order(schema::mastodon_posts::created_at.desc())
            .select(schema::mastodon_posts::post_id)
            .first::<String>(&mut self.db_connection)
            .optional()?;

        // Get the latest posts from Mastodon.
        // If there is no last synced post ID, we will only get the latest post.
        // Otherwise, we will get all posts since the last synced post.
        tracing::info!("Getting latest posts from Mastodon...");
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

            let new_mastodon_post = models::NewMastodonPost::new(&initial_post, None, None);
            diesel::insert_into(schema::mastodon_posts::dsl::mastodon_posts)
                .values(&new_mastodon_post)
                .execute(&mut self.db_connection)?;

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

            let mut post_sync = bsky::BlueSkyPostSync {
                config: self.config.clone(),
                db_connection: &mut self.db_connection,
                bsky_auth: self.bsky_auth.clone(),
                mastodon_account: account.json.clone(),
                mastodon_status: post_item.clone(),
                post_item: app_bsky::feed::Post::new("", post_item.created_at.clone(), None)
            };

            let sync_result = post_sync.sync_post().await;

            match sync_result {
                Ok(_) => {
                    tracing::info!("Post '{}' processed successfully.", post_item.id);
                }
                Err(e) => {
                    tracing::error!("Failed to process post '{}': {:#?}", post_item.id, e);

                    let source_error = e.source();

                    if let Some(source_error) = source_error {
                        tracing::error!("Source error: {:#?}", source_error);
                    }
                }
            }
        }

        let cached_files_to_delete = schema::cached_files::table
            .select(crate::models::CachedFile::as_select())
            .load(&mut self.db_connection)?;

        if cached_files_to_delete.len() > 0 {
            tracing::info!("Deleting cached files during sync...");

            for cached_file in cached_files_to_delete {
                tracing::info!("Deleting cached file '{}'.", cached_file.file_path);
                crate::models::remove_cached_file(&cached_file, &mut self.db_connection).await?;
            }
        }

        Ok(())
    }
}

/// Create a new HTTP client for the FediProto Sync application.
///
/// ## Arguments
///
/// * `config` - The environment variables for the FediProto Sync application.
pub fn create_http_client(
    config: &FediProtoSyncEnvVars
) -> Result<reqwest::Client, crate::error::Error> {
    reqwest::Client::builder()
        .user_agent(config.user_agent.clone())
        .build()
        .map_err(|e| {
            crate::error::Error::with_source(
                "Failed to create HTTP client.",
                crate::error::ErrorKind::HttpClientCreationError,
                Box::new(e)
            )
        })
}
