/// BlueSky operations for syncing posts.
pub mod bsky_post_sync;
/// CLI operations for the application.
pub mod cli;
/// Core operations for the application.
pub mod core;
/// Utilities for working with images.
pub mod img_utils;
/// Mastodon operations.
pub mod mastodon;

use anyhow::Result;
use fediproto_sync_lib::{
    GIT_VERSION,
    config::{FediProtoSyncConfig, FediProtoSyncMode},
};

/// Handles the `run` command.
///
/// ## Arguments
///
/// * `config` - The config for the app.
pub async fn handle_run_command(config: FediProtoSyncConfig) -> Result<()> {
    // Set up unbounded channels for shutdown and error signals.
    let (shutdown_send, mut shutdown_recv) = tokio::sync::mpsc::unbounded_channel();
    let (core_sig_error_send, mut core_sig_error_recv) = tokio::sync::mpsc::unbounded_channel();

    // Set up signal handlers for SIGTERM and SIGQUIT.
    #[cfg(target_family = "unix")]
    let mut sig_term = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    #[cfg(target_family = "unix")]
    let mut sig_quit = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::quit())?;

    #[cfg(target_family = "windows")]
    let mut sig_term = tokio::signal::windows::ctrl_c()?;
    #[cfg(target_family = "windows")]
    let mut sig_quit = tokio::signal::windows::ctrl_shutdown()?;

    tracing::info!("FediProto Sync - v{}", GIT_VERSION);
    tracing::info!("Press Ctrl+C to shutdown at any time...");

    let database_url = config.database_url.clone();

    let db_connection_pool = fediproto_sync_db::create_database_connection(&database_url)?;
    tracing::info!("Connected to database.");

    let db_connection_main = &mut db_connection_pool.get()?;

    fediproto_sync_db::core::run_migrations(db_connection_main)?;

    let cached_tokens_exist =
        fediproto_sync_db::operations::get_cached_service_token_by_service_name(
            db_connection_main,
            "mastodon",
        )?
        .is_some();

    match config.mode == FediProtoSyncMode::Auth || !cached_tokens_exist {
        true => {
            let config_auth = config.clone();
            let db_connection_pool_auth = db_connection_pool.clone();

            // Spawn the auth web server.
            tokio::spawn(async move {
                let fediproto_auth_web_server =
                    fediproto_sync_auth_ui::FediProtoSyncWebServer::new(
                        &config_auth,
                        db_connection_pool_auth,
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

/// Handles the `generate-token-encryption-key` command.
pub fn handle_generate_token_key_command() -> Result<()> {
    let encryption_keys = fediproto_sync_lib::crypto::generate_token_encryption_key()?;

    println!("{}", encryption_keys);

    Ok(())
}
