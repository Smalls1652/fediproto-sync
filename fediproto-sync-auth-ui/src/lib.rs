/// Authentication operations for Mastodon and BlueSky.
pub mod auth;
/// Error types for the FediProtoSync authentication web server.
pub mod error;
/// Endpoints for the FediProtoSync authentication web server.
pub mod web;

use async_session::MemoryStore;
use axum::{Router, extract::FromRef, routing::get};
use diesel::r2d2::{ConnectionManager, Pool};
use fediproto_sync_db::AnyConnection;
use fediproto_sync_lib::{config::FediProtoSyncConfig, error::FediProtoSyncError};
use oauth2::basic::BasicClient;
use web::{mastodon_auth, root};

/// Represents the "app state" for the FediProtoSync web server.
#[derive(FromRef, Clone)]
pub struct FediProtoSyncWebServerAppState {
    /// The FediProtoSync configuration.
    pub config: FediProtoSyncConfig,

    /// The database connection pool.
    pub db_pool: Pool<ConnectionManager<AnyConnection>>,

    /// The Mastodon OAuth2 client.
    pub mastodon_oauth_client: BasicClient,

    /// The memory store for storing session data.
    pub memory_store: MemoryStore,
}

/// Represents the FediProtoSync web server.
pub struct FediProtoSyncWebServer {
    /// The FediProtoSync configuration.
    pub config: FediProtoSyncConfig,

    /// The database connection pool.
    pub db_pool: Pool<ConnectionManager<AnyConnection>>,
}

impl FediProtoSyncWebServer {
    /// Create a new instance of the `FediProtoSyncWebServer`.
    ///
    /// ## Arguments
    ///
    /// * `config` - The FediProtoSync configuration.
    /// * `db_pool` - The database connection pool.
    pub fn new(
        config: &FediProtoSyncConfig,
        db_pool: Pool<ConnectionManager<AnyConnection>>,
    ) -> Result<Self, FediProtoSyncError> {
        let config = config.clone();

        Ok(Self { config, db_pool })
    }

    /// Run the web server.
    pub async fn run(&self) -> Result<(), FediProtoSyncError> {
        // Get the bind address and port.
        let bind_address = self
            .config
            .auth_server_address
            .clone()
            .unwrap_or_else(|| "localhost".to_string());
        let bind_port = self.config.auth_server_port.unwrap_or_else(|| 3000);

        let full_bind_address = format!("{}:{}", bind_address, bind_port);

        // Create the TCP listener on the specified address and port.
        tracing::info!("Starting the web server on '{}'...", full_bind_address);
        let listener = tokio::net::TcpListener::bind(&full_bind_address)
            .await
            .map_err(|_| FediProtoSyncError::WebServerError)?;

        // Create the memory store.
        let memory_store = MemoryStore::new();

        // Get the Mastodon redirect URI.
        let mastodon_redirect_uri = self.config.mastodon_redirect_uri.clone();

        // Create the app state.
        let app_state = FediProtoSyncWebServerAppState {
            config: self.config.clone(),
            db_pool: self.db_pool.clone(),
            mastodon_oauth_client: auth::mastodon::get_mastodon_oauth_client(
                &self.config,
                &mastodon_redirect_uri,
            )?,
            memory_store,
        };

        // Create the router and define the routes.
        let router = Router::new()
            .route("/", get(root::root_endpoint))
            .route("/auth/mastodon/login", get(mastodon_auth::login_endpoint))
            .route(
                "/auth/mastodon/authorized",
                get(mastodon_auth::authorized_endpoint),
            )
            .route(
                "/auth/mastodon/already_authorized",
                get(mastodon_auth::already_authorized_endpoint),
            )
            .with_state(app_state)
            .into_make_service();

        tracing::info!(
            "\nGo to this URL to setup authentication:\n\nhttp://{}:{}",
            bind_address,
            bind_port
        );

        // Serve the web server.
        axum::serve(listener, router)
            .await
            .map_err(|_| FediProtoSyncError::WebServerError)?;

        Ok(())
    }
}
