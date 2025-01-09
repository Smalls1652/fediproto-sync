/// BlueSky operations for syncing posts.
mod bsky;
/// Core operations for the application.
mod core;
/// Utilities for working with images.
mod img_utils;
/// Mastodon operations.
mod mastodon;

use anyhow::Result;
use fediproto_sync_lib::{
    config::{FediProtoSyncConfig, FediProtoSyncMode},
    GIT_VERSION
};

/// The main entrypoint for the FediProtoSync application.
#[tokio::main]
async fn main() -> Result<()> {
    let rust_log_result = std::env::var("RUST_LOG");

    match rust_log_result {
        Ok(_) => (),
        Err(_) => {
            std::env::set_var("RUST_LOG", "info");
        }
    }

    // Set up unbounded channels for shutdown and error signals.
    let (shutdown_send, mut shutdown_recv) = tokio::sync::mpsc::unbounded_channel();
    let (core_sig_error_send, mut core_sig_error_recv) = tokio::sync::mpsc::unbounded_channel();

    // Set up signal handlers for SIGTERM and SIGQUIT.
    let mut sig_term = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut sig_quit = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::quit())?;

    // Set up tracing subscriber for logging.
    tracing_subscriber::fmt()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_target(true)
        .with_thread_ids(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("FediProto Sync - v{}", GIT_VERSION);
    tracing::info!("Press Ctrl+C to shutdown at any time...");

    // Load environment variables from the .env file for the specified environment,
    // if it exists.
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

    let config_result = FediProtoSyncConfig::new();

    let config = match config_result {
        Ok(config) => config,
        Err(e) => {
            tracing::error!("{}", e);

            std::process::exit(1);
        }
    };

    let database_url = config.database_url.clone();

    let db_connection_pool = fediproto_sync_db::create_database_connection(&database_url)?;
    tracing::info!("Connected to database.");

    let db_connection_main = &mut db_connection_pool.get()?;

    fediproto_sync_db::core::run_migrations(db_connection_main)?;

    let cached_tokens_exist = fediproto_sync_db::operations::get_cached_service_token_by_service_name(db_connection_main, "mastodon")?.is_some();

    match config.mode == FediProtoSyncMode::Auth || !cached_tokens_exist {
        true => {
            let config_auth = config.clone();
            let db_connection_pool_auth = db_connection_pool.clone();

            // Spawn the auth web server.
            tokio::spawn(async move {
                let fediproto_auth_web_server =
                    fediproto_sync_auth_ui::FediProtoSyncWebServer::new(
                        &config_auth,
                        db_connection_pool_auth
                    )
                    .unwrap();

                let result = fediproto_auth_web_server.run().await;

                match result {
                    Ok(_) => {
                        tracing::info!("Auth server completed successfully.");
                    }
                    Err(e) => {
                        tracing::error!("Auth server failed: {}", e);

                        core_sig_error_send.send(()).unwrap();
                    }
                }
            });
        }

        false => {
            let config_core = config.clone();
            let db_connection_pool_core = db_connection_pool.clone();

            // Spawn the core loop for running the syncs.
            tokio::spawn(async move {
                let mut fediprotosync_loop =
                    core::FediProtoSyncLoop::new(&config_core, db_connection_pool_core)
                        .await
                        .unwrap();

                let result = fediprotosync_loop.run_loop().await;

                match result {
                    Ok(_) => {
                        tracing::info!("FediProto Sync completed successfully.");
                    }
                    Err(e) => {
                        tracing::error!("FediProto Sync failed: {}", e);

                        core_sig_error_send.send(()).unwrap();
                    }
                }
            });
        }
    };

    // Wait for signals to be received.
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::warn!("Received Ctrl+C, shutting down...");
            shutdown_send.send(()).unwrap();
        },

        _ = core_sig_error_recv.recv() => {
            tracing::error!("An error occurred. Shutting down...");
            shutdown_send.send(()).unwrap();
        },

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
