mod bsky;
mod core;
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
    pub fn new() -> Result<Self, std::env::VarError> {
        let mastodon_server = std::env::var("MASTODON_SERVER")?;
        let mastodon_access_token = std::env::var("MASTODON_ACCESS_TOKEN")?;
        let bluesky_pds_server = std::env::var("BLUESKY_PDS_SERVER")?;
        let bluesky_handle = std::env::var("BLUESKY_HANDLE")?;
        let bluesky_app_password = std::env::var("BLUESKY_APP_PASSWORD")?;
        let sync_interval = std::time::Duration::from_secs(
            std::env::var("SYNC_INTERVAL_SECONDS")
                .unwrap_or("30".to_string())
                .parse::<u64>()
                .unwrap()
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

    let config = FediProtoSyncEnvVars::new()?;

    tokio::spawn(async move {
        core::run(config).await.unwrap();
    });

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl-C, shutting down...");
            shutdown_send.send(()).unwrap();
        },

        _ = sig_term.recv() => {
            println!("Received SIGTERM, shutting down...");
            shutdown_send.send(()).unwrap();
        },

        _ = sig_quit.recv() => {
            println!("Received SIGQUIT, shutting down...");
            shutdown_send.send(()).unwrap();
        },

        _ = shutdown_recv.recv() => {
            println!("Shutting down...");

            return Ok(());
        }
    }

    Ok(())
}
