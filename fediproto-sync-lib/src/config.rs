use clap::ValueEnum;
use openssl::{
    pkey::{Private, Public},
    rsa::Rsa,
};

#[allow(unused_imports)]
use crate::{GIT_VERSION, error::FediProtoSyncError};

/*
static FEDIPROTO_SYNC_MODE_ENV_VAR: &str = "FEDIPROTO_SYNC_MODE";
static AUTH_SERVER_ADDRESS_ENV_VAR: &str = "AUTH_SERVER_ADDRESS";
static AUTH_SERVER_PORT_ENV_VAR: &str = "AUTH_SERVER_PORT";
static DATABASE_TYPE_ENV_VAR: &str = "DATABASE_TYPE";
static DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
static TOKEN_ENCRYPTION_PRIVATE_KEY_ENV_VAR: &str = "TOKEN_ENCRYPTION_PRIVATE_KEY";
static TOKEN_ENCRYPTION_PUBLIC_KEY_ENV_VAR: &str = "TOKEN_ENCRYPTION_PUBLIC_KEY";
static USER_AGENT_ENV_VAR: &str = "USER_AGENT";
static MASTODON_SERVER_ENV_VAR: &str = "MASTODON_SERVER";
static MASTODON_CLIENT_ID_ENV_VAR: &str = "MASTODON_CLIENT_ID";
static MASTODON_CLIENT_SECRET_ENV_VAR: &str = "MASTODON_CLIENT_SECRET";
static MASTODON_REDIRECT_URI_ENV_VAR: &str = "MASTODON_REDIRECT_URI";
static BLUESKY_PDS_SERVER_ENV_VAR: &str = "BLUESKY_PDS_SERVER";
static BLUESKY_HANDLE_ENV_VAR: &str = "BLUESKY_HANDLE";
static BLUESKY_APP_PASSWORD_ENV_VAR: &str = "BLUESKY_APP_PASSWORD";
static SYNC_INTERVAL_SECONDS_ENV_VAR: &str = "SYNC_INTERVAL_SECONDS";
static BLUESKY_VIDEO_ALWAYS_FALLBACK_ENV_VAR: &str = "BLUESKY_VIDEO_ALWAYS_FALLBACK";
static MASTODON_ALLOW_UNLISTED_POSTS_ENV_VAR: &str = "MASTODON_ALLOW_UNLISTED_POSTS";
*/

/// Config values for configuring the FediProtoSync
/// application.
#[derive(Debug, Clone)]
pub struct FediProtoSyncConfig {
    /// The mode to run the application in.
    ///
    /// **Environment variable:** `FEDIPROTO_SYNC_MODE`
    pub mode: FediProtoSyncMode,

    /// The address to bind the auth server to.
    ///
    /// **Environment variable:** `AUTH_SERVER_ADDRESS`
    pub auth_server_address: Option<String>,

    /// The port to bind the auth server to.
    ///
    /// **Environment variable:** `AUTH_SERVER_PORT`
    pub auth_server_port: Option<u16>,

    /// The type of database to use.
    ///
    /// **Environment variable:** `DATABASE_TYPE`
    pub database_type: DatabaseType,

    /// The URL/path to the database.
    ///
    /// **Environment variable:** `DATABASE_URL`
    pub database_url: String,

    /// The private key to use for token encryption.
    ///
    /// **Environment variable:** `TOKEN_ENCRYPTION_PRIVATE_KEY`
    pub token_encryption_private_key: openssl::rsa::Rsa<openssl::pkey::Private>,

    /// The public key to use for token encryption.
    ///
    /// **Environment variable:** `TOKEN_ENCRYPTION_PUBLIC_KEY`
    pub token_encryption_public_key: openssl::rsa::Rsa<openssl::pkey::Public>,

    /// User-Agent string to use for HTTP requests.
    ///
    /// **Environment variable:** `USER_AGENT`
    pub user_agent: String,

    /// The Mastodon server URL to connect to.
    ///
    /// **Environment variable:** `MASTODON_SERVER`
    pub mastodon_server: String,

    /// The client ID for the Mastodon application.
    ///
    /// **Environment variable:** `MASTODON_CLIENT_ID`
    pub mastodon_client_id: String,

    /// The client secret for the Mastodon application.
    ///
    /// **Environment variable:** `MASTODON_CLIENT_SECRET`
    pub mastodon_client_secret: String,

    /// The redirect URI for the Mastodon application.
    ///
    /// **Environment variable:** `MASTODON_REDIRECT_URI`
    pub mastodon_redirect_uri: String,

    /// The BlueSky PDS URL to connect to.
    ///
    /// **Environment variable:** `BLUESKY_PDS_SERVER`
    pub bluesky_pds_server: String,

    /// The BlueSky handle to use for authentication.
    ///
    /// **Environment variable:** `BLUESKY_HANDLE`
    pub bluesky_handle: String,

    /// The BlueSky app password to use for authentication.
    ///
    /// **Environment variable:** `BLUESKY_APP_PASSWORD`
    pub bluesky_app_password: String,

    /// The interval, in seconds, to sync posts.
    ///
    /// **Environment variable:** `SYNC_INTERVAL_SECONDS`
    pub sync_interval: std::time::Duration,

    /// Whether to always fallback to the video URL for BlueSky posts.
    ///
    /// **Environment variable:** `BLUESKY_VIDEO_ALWAYS_FALLBACK`
    pub bluesky_video_always_fallback: bool,

    /// Whether to allow unlisted posts from Mastodon to sync to BlueSky.
    ///
    /// **Environment variable:** `MASTODON_ALLOW_UNLISTED_POSTS`
    pub mastodon_allow_unlisted_posts: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum FediProtoSyncMode {
    #[value(name = "auth")]
    Auth,

    #[value(name = "normal")]
    Normal,
}

/// The type of database to use.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DatabaseType {
    /// PostgreSQL database
    #[value(name = "Postgres")]
    Postgres,

    /// SQLite database
    #[value(name = "SQLite")]
    SQLite,
}

/// Decode a Base64 string into a private key.
///
/// ## Arguments
///
/// * `key` - A Base64 encoded string of a private key.
pub fn get_token_encryption_private_key(key: &str) -> Result<Rsa<Private>, FediProtoSyncError> {
    let token_encryption_private_key =
        openssl::base64::decode_block(key).map_err(|_| FediProtoSyncError::KeyLoadError)?;

    openssl::rsa::Rsa::private_key_from_pem(&token_encryption_private_key)
        .map_err(|_| FediProtoSyncError::KeyLoadError)
}

/// Decode a Base64 string into a public key.
///
/// ## Arguments
///
/// * `key` - A Base64 encoded string of a public key.
pub fn get_token_encryption_public_key(key: &str) -> Result<Rsa<Public>, FediProtoSyncError> {
    let token_encryption_public_key =
        openssl::base64::decode_block(&key).map_err(|_| FediProtoSyncError::KeyLoadError)?;

    openssl::rsa::Rsa::public_key_from_pem(&token_encryption_public_key)
        .map_err(|_| FediProtoSyncError::KeyLoadError)
}
