mod bsky;
mod core;
mod error;
mod mastodon;
mod models;
mod schema;

/// The environment variable values for configuring the FediProtoSync application.
#[derive(Debug, Clone)]
pub struct FediProtoSyncEnvVars {
    /// The URL/path to the SQLite database file.
    pub database_url: String,

    /// The Mastodon server URL to connect to.
    pub mastodon_server: String,

    /// The Mastodon access token to use for authentication.
    pub mastodon_access_token: String,

    /// The BlueSky PDS URL to connect to.
    pub bluesky_pds_server: String,

    /// The BlueSky handle to use for authentication.
    pub bluesky_handle: String,

    /// The BlueSky app password to use for authentication.
    pub bluesky_app_password: String,

    /// The interval, in seconds, to sync posts.
    pub sync_interval: std::time::Duration,

    /// Whether to always fallback to the video URL for BlueSky posts.
    pub bluesky_video_always_fallback: bool
}

impl FediProtoSyncEnvVars {
    /// Create a new instance of the `FediProtoSyncEnvVars` struct.
    pub fn new() -> Result<Self, crate::error::Error> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|e| {
                crate::error::Error::with_source(
                    "Failed to read DATABASE_URL environment variable.",
                    crate::error::ErrorKind::EnvironmentVariableError,
                    Box::new(e)
                )
            })?;

        let mastodon_server = std::env::var("MASTODON_SERVER")
            .map_err(|e| {
                crate::error::Error::with_source(
                    "Failed to read MASTODON_SERVER environment variable.",
                    crate::error::ErrorKind::EnvironmentVariableError,
                    Box::new(e)
                )
            })?;

        let mastodon_access_token = std::env::var("MASTODON_ACCESS_TOKEN")
            .map_err(|e| {
                crate::error::Error::with_source(
                    "Failed to read MASTODON_ACCESS_TOKEN environment variable.",
                    crate::error::ErrorKind::EnvironmentVariableError,
                    Box::new(e)
                )
            })?;

        let bluesky_pds_server = std::env::var("BLUESKY_PDS_SERVER")
            .map_err(|e| {
                crate::error::Error::with_source(
                    "Failed to read BLUESKY_PDS_SERVER environment variable.",
                    crate::error::ErrorKind::EnvironmentVariableError,
                    Box::new(e)
                )
            })?;

        let bluesky_handle = std::env::var("BLUESKY_HANDLE")
            .map_err(|e| {
                crate::error::Error::with_source(
                    "Failed to read BLUESKY_HANDLE environment variable.",
                    crate::error::ErrorKind::EnvironmentVariableError,
                    Box::new(e)
                )
            })?;
        let bluesky_app_password = std::env::var("BLUESKY_APP_PASSWORD")
            .map_err(|e| {
                crate::error::Error::with_source(
                    "Failed to read BLUESKY_APP_PASSWORD environment variable.",
                    crate::error::ErrorKind::EnvironmentVariableError,
                    Box::new(e)
                )
            })?;

        let sync_interval = std::time::Duration::from_secs(
            std::env::var("SYNC_INTERVAL_SECONDS")
                .unwrap_or("300".to_string())
                .parse::<u64>()
                .map_err(|e| {
                    crate::error::Error::with_source(
                        "Failed to parse the SYNC_INTERVAL_SECONDS environment variable.",
                        crate::error::ErrorKind::EnvironmentVariableError,
                        Box::new(e)
                    )
                })?
        );

        let bluesky_video_always_fallback = std::env::var("BLUESKY_VIDEO_ALWAYS_FALLBACK")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .map_err(|e| {
                crate::error::Error::with_source(
                    "Failed to parse the BLUESKY_VIDEO_ALWAYS_FALLBACK environment variable.",
                    crate::error::ErrorKind::EnvironmentVariableError,
                    Box::new(e)
                )
            })?;

        Ok(Self {
            database_url,
            mastodon_server,
            mastodon_access_token,
            bluesky_pds_server,
            bluesky_handle,
            bluesky_app_password,
            sync_interval,
            bluesky_video_always_fallback
        })
    }
}

/// The main entrypoint for the FediProtoSync application.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up unbounded channels for shutdown and error signals.
    let (shutdown_send, mut shutdown_recv) = tokio::sync::mpsc::unbounded_channel();
    let (sig_error_send, mut sig_error_recv) = tokio::sync::mpsc::unbounded_channel();

    // Set up signal handlers for SIGTERM and SIGQUIT.
    let mut sig_term = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut sig_quit = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::quit())?;

    // Set up tracing subscriber for logging.
    let trace_subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .with_thread_ids(false)
        .finish();
    tracing::subscriber::set_global_default(trace_subscriber)?;

    // Load environment variables from the .env file for the specified environment, if it exists.
    let root_app_dir = std::env::current_dir()?;
    let environment_name_result = std::env::var("FEDIPROTO_SYNC_ENVIRONMENT");
    let environment_name = match environment_name_result {
        Ok(environment_name) => environment_name,
        Err(_) => "Production".to_string()
    };

    let env_vars_path = root_app_dir.join(format!("{}.env", environment_name.to_lowercase()));

    if env_vars_path.exists() {
        let _ = dotenvy::from_path(env_vars_path)?;
    }

    let config_result = FediProtoSyncEnvVars::new();

    let config = match config_result {
        Ok(config) => config,
        Err(e) => {
            tracing::error!("{}", e.message);

            std::process::exit(1);
        }
    };

    // Spawn the core loop for running the syncs.
    tokio::spawn(async move {
        let mut fediprotosync_loop = core::FediProtoSyncLoop::new(&config).await.unwrap();
        
        let result = fediprotosync_loop.run_loop().await;

        match result {
            Ok(_) => {
                tracing::info!("FediProto Sync completed successfully.");
            }
            Err(e) => {
                tracing::error!("FediProto Sync failed: {}", e.message);

                sig_error_send.send(()).unwrap();
            }
        }
    });

    // Wait for signals to be received.
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::warn!("Received Ctrl+C, shutting down...");
            shutdown_send.send(()).unwrap();
        },

        _ = sig_error_recv.recv() => {
            tracing::error!("An error occurred. Shutting down...");
            shutdown_send.send(()).unwrap();
        }

        _ = sig_term.recv() => {
            tracing::warn!("Received SIGTERM, shutting down...");
            shutdown_send.send(()).unwrap();
        },

        _ = sig_quit.recv() => {
            tracing::warn!("Received SIGQUIT, shutting down...");
            shutdown_send.send(()).unwrap();
        },

        _ = shutdown_recv.recv() => {
            tracing::debug!("Received shutdown signal, shutting down...");

            return Ok(());
        }
    }

    Ok(())
}
