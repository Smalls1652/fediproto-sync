use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use fediproto_sync_lib::config::{DatabaseType, FediProtoSyncConfig, FediProtoSyncMode};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CliSubcommands {
    /// Run the app.
    Run(RunArgs),

    /// Generate keypair for token encryption.
    GenerateTokenEncryptionKey,
}

#[derive(Args, Debug, Clone)]
pub struct RunArgs {
    /// The mode to run the application in.
    #[arg(
        long = "mode",
        env = "FEDIPROTO_SYNC_MODE",
        default_value_t = FediProtoSyncMode::Normal,
        value_enum
    )]
    pub mode: FediProtoSyncMode,

    /// The address to bind the auth server to.
    #[arg(
        long = "auth-server-address",
        env = "AUTH_SERVER_ADDRESS",
        default_value = "0.0.0.0"
    )]
    pub auth_server_address: String,

    /// The port to bind the auth server to.
    #[arg(
        long = "auth-server-port",
        env = "AUTH_SERVER_PORT",
        default_value = "3000"
    )]
    pub auth_server_port: u16,

    /// The type of database to use.
    #[arg(
        long = "database-type",
        env = "DATABASE_TYPE",
        default_value_t = DatabaseType::Postgres,
        value_enum
    )]
    pub database_type: DatabaseType,

    /// The URL/path to the database.
    #[arg(long = "database-url", env = "DATABASE_URL", required = true)]
    pub database_url: String,

    /// The private key to use for token encryption.
    #[arg(
        long = "token-encryption-private-key",
        env = "TOKEN_ENCRYPTION_PRIVATE_KEY",
        required = true,
        value_parser = fediproto_sync_lib::config::get_token_encryption_private_key
    )]
    pub token_encryption_private_key: openssl::rsa::Rsa<openssl::pkey::Private>,

    /// The public key to use for token encryption.
    #[arg(
        long = "token-encryption-public-key",
        env = "TOKEN_ENCRYPTION_PUBLIC_KEY",
        required = true,
        value_parser = fediproto_sync_lib::config::get_token_encryption_public_key
    )]
    pub token_encryption_public_key: openssl::rsa::Rsa<openssl::pkey::Public>,

    /// User-Agent string to use for HTTP requests.
    #[arg(
        long = "user-agent",
        env = "USER_AGENT",
        default_value = "FediProto Sync"
    )]
    pub user_agent: String,

    /// The Mastodon server URL to connect to.
    #[arg(long = "mastodon-server", env = "MASTODON_SERVER", required = true)]
    pub mastodon_server: String,

    /// The client ID for the Mastodon application.
    #[arg(
        long = "mastodon-client-id",
        env = "MASTODON_CLIENT_ID",
        required = true
    )]
    pub mastodon_client_id: String,

    /// The client secret for the Mastodon application.
    #[arg(
        long = "mastodon-client-secret",
        env = "MASTODON_CLIENT_SECRET",
        required = true
    )]
    pub mastodon_client_secret: String,

    /// The redirect URI for the Mastodon application.
    #[arg(
        long = "mastodon-redirect-uri",
        env = "MASTODON_REDIRECT_URI",
        required = true
    )]
    pub mastodon_redirect_uri: String,

    /// The BlueSky PDS URL to connect to.
    #[arg(
        long = "bluesky-pds-server",
        env = "BLUESKY_PDS_SERVER",
        required = true
    )]
    pub bluesky_pds_server: String,

    /// The BlueSky handle to use for authentication.
    #[arg(long = "bluesky-handle", env = "BLUESKY_HANDLE", required = true)]
    pub bluesky_handle: String,

    /// The BlueSky app password to use for authentication.
    #[arg(
        long = "bluesky-app-password",
        env = "BLUESKY_APP_PASSWORD",
        required = true
    )]
    pub bluesky_app_password: String,

    /// The interval, in seconds, to sync posts.
    #[arg(
        long = "sync-interval",
        env = "SYNC_INTERVAL_SECONDS",
        default_value = "300",
        value_parser = sync_interval_parser
    )]
    pub sync_interval: std::time::Duration,

    /// Whether to always fallback to the video URL for BlueSky posts.
    #[arg(
        long = "bluesky-video-always-fallback",
        env = "BLUESKY_VIDEO_ALWAYS_FALLBACK",
        default_value_t = false
    )]
    pub bluesky_video_always_fallback: bool,

    /// Whether to allow unlisted posts from Mastodon to sync to BlueSky.
    #[arg(
        long = "mastodon-allow-unlisted-posts",
        env = "MASTODON_ALLOW_UNLISTED_POSTS",
        default_value_t = false
    )]
    pub mastodon_allow_unlisted_posts: bool,
}

impl Into<FediProtoSyncConfig> for RunArgs {
    fn into(self) -> FediProtoSyncConfig {
        FediProtoSyncConfig {
            mode: self.mode.to_owned(),
            auth_server_address: Some(self.auth_server_address.to_owned()),
            auth_server_port: Some(self.auth_server_port.to_owned()),
            database_type: self.database_type.to_owned(),
            database_url: self.database_url.to_owned(),
            token_encryption_private_key: self.token_encryption_private_key.to_owned(),
            token_encryption_public_key: self.token_encryption_public_key.to_owned(),
            user_agent: self.user_agent.to_owned(),
            mastodon_server: self.mastodon_server.to_owned(),
            mastodon_client_id: self.mastodon_client_id.to_owned(),
            mastodon_client_secret: self.mastodon_client_secret.to_owned(),
            mastodon_redirect_uri: self.mastodon_redirect_uri.to_owned(),
            bluesky_pds_server: self.bluesky_pds_server.to_owned(),
            bluesky_handle: self.bluesky_handle.to_owned(),
            bluesky_app_password: self.bluesky_app_password.to_owned(),
            sync_interval: self.sync_interval.to_owned(),
            bluesky_video_always_fallback: self.bluesky_video_always_fallback.to_owned(),
            mastodon_allow_unlisted_posts: self.mastodon_allow_unlisted_posts.to_owned(),
        }
    }
}

fn sync_interval_parser(value: &str) -> Result<std::time::Duration> {
    let value = value.parse::<u64>()?;

    Ok(std::time::Duration::from_secs(value))
}
