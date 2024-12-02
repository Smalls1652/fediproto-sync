mod bsky;
mod core;
mod error;
mod mastodon;
mod models;
mod schema;

#[derive(Debug, Clone)]
struct FediProtoSyncEnvVars {
    pub mastodon_server: String,
    pub mastodon_access_token: String,
    pub bluesky_pds_server: String,
    pub bluesky_handle: String,
    pub bluesky_app_password: String,
    pub sync_interval: std::time::Duration
}

impl FediProtoSyncEnvVars {
    pub fn new() -> Result<Self, crate::error::Error> {
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
                .unwrap_or("30".to_string())
                .parse::<u64>()
                .map_err(|e| {
                    crate::error::Error::with_source(
                        "Failed to parse the SYNC_INTERVAL_SECONDS environment variable.",
                        crate::error::ErrorKind::EnvironmentVariableError,
                        Box::new(e)
                    )
                })?
        );

        Ok(Self {
            mastodon_server,
            mastodon_access_token,
            bluesky_pds_server,
            bluesky_handle,
            bluesky_app_password,
            sync_interval
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (shutdown_send, mut shutdown_recv) = tokio::sync::mpsc::unbounded_channel();

    let (sig_error_send, mut sig_error_recv) = tokio::sync::mpsc::unbounded_channel();

    let mut sig_term = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut sig_quit = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::quit())?;

    let trace_subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .with_thread_ids(true)
        .finish();

    tracing::subscriber::set_global_default(trace_subscriber)?;

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

    tokio::spawn(async move {
        let result = core::run(config).await;

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
