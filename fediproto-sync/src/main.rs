use anyhow::Result;
use clap::Parser;
use fediproto_sync_lib::config::FediProtoSyncConfig;

use fediproto_sync::cli::{Cli, CliSubcommands};

// Use Jemalloc for *nix-based systems, excluding macOS.
#[cfg(all(target_family = "unix", not(target_os = "macos")))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

// Use snmalloc for macOS-based systems.
#[cfg(all(target_family = "unix", target_os = "macos"))]
#[global_allocator]
static GLOBAL: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

/// The main entrypoint for the FediProtoSync application.
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from the .env file for the specified environment,
    // if it exists.
    let root_app_dir = std::env::current_dir()?;
    let environment_name_result = std::env::var("FEDIPROTO_SYNC_ENVIRONMENT");
    let environment_name = match environment_name_result {
        Ok(environment_name) => environment_name,
        Err(_) => "Production".to_string(),
    };

    let env_vars_path = root_app_dir.join(format!("{}.env", environment_name.to_lowercase()));

    if env_vars_path.exists() {
        let _ = dotenvy::from_path(env_vars_path)?;
    }

    let cli = Cli::parse();

    // Set up tracing subscriber for logging.
    tracing_subscriber::fmt()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_target(true)
        .with_thread_ids(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    match cli.command {
        CliSubcommands::Run(run_args) => {
            let config: FediProtoSyncConfig = run_args.into();

            fediproto_sync::handle_run_command(config).await
        }

        CliSubcommands::GenerateTokenEncryptionKey => {
            fediproto_sync::handle_generate_token_key_command()
        }
    }
}
