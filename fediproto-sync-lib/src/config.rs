use crate::{error::FediProtoSyncError, GIT_VERSION};

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
    pub bluesky_video_always_fallback: bool
}

impl FediProtoSyncConfig {
    /// Create a new instance of the `FediProtoSyncConfig` struct.
    pub fn new() -> Result<Self, FediProtoSyncError> {
        // Read 'FEDIPROTO_SYNC_MODE' environment variable.
        let mode = std::env::var(FEDIPROTO_SYNC_MODE_ENV_VAR)
            .unwrap_or("normal".to_string())
            .parse::<FediProtoSyncMode>()
            .map_err(|_| {
                FediProtoSyncError::EnvironmentVariableError(
                    FEDIPROTO_SYNC_MODE_ENV_VAR.to_string()
                )
            })?;

        // Read 'AUTH_SERVER_ADDRESS' environment variable.
        let auth_server_address = match mode {
            FediProtoSyncMode::Auth => Some(
                std::env::var(AUTH_SERVER_ADDRESS_ENV_VAR)
                    .unwrap_or_else(|_| "localhost".to_string())
            ),

            FediProtoSyncMode::Normal => None
        };

        // Read 'AUTH_SERVER_PORT' environment variable.
        let auth_server_port = match mode {
            FediProtoSyncMode::Auth => Some(
                std::env::var(AUTH_SERVER_PORT_ENV_VAR)
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse::<u16>()
                    .map_err(|_| {
                        FediProtoSyncError::EnvironmentVariableParseError(
                            AUTH_SERVER_PORT_ENV_VAR.to_string()
                        )
                    })?
            ),

            FediProtoSyncMode::Normal => None
        };

        // Read 'DATABASE_TYPE' environment variable.
        let database_type = std::env::var(DATABASE_TYPE_ENV_VAR)
            .unwrap_or("Postgres".to_string())
            .parse::<DatabaseType>()?;

        // Read 'DATABASE_URL' environment variable.
        let database_url = std::env::var(DATABASE_URL_ENV_VAR).map_err(|_| {
            FediProtoSyncError::EnvironmentVariableError(DATABASE_URL_ENV_VAR.to_string())
        })?;

        // Read 'TOKEN_ENCRYPTION_PRIVATE_KEY' environment variable.
        let token_encryption_private_key = std::env::var(TOKEN_ENCRYPTION_PRIVATE_KEY_ENV_VAR)
            .map_err(|_| {
                FediProtoSyncError::EnvironmentVariableError(
                    TOKEN_ENCRYPTION_PRIVATE_KEY_ENV_VAR.to_string()
                )
            })?;

        let token_encryption_private_key =
            openssl::base64::decode_block(&token_encryption_private_key).map_err(|_| {
                FediProtoSyncError::EnvironmentVariableError(
                    TOKEN_ENCRYPTION_PRIVATE_KEY_ENV_VAR.to_string()
                )
            })?;

        let token_encryption_private_key = openssl::rsa::Rsa::private_key_from_pem(
            &token_encryption_private_key
        )
        .map_err(|_| {
            FediProtoSyncError::EnvironmentVariableError(
                TOKEN_ENCRYPTION_PRIVATE_KEY_ENV_VAR.to_string()
            )
        })?;

        // Read 'TOKEN_ENCRYPTION_PUBLIC_KEY' environment variable.
        let token_encryption_public_key = std::env::var(TOKEN_ENCRYPTION_PUBLIC_KEY_ENV_VAR)
            .map_err(|_| {
                FediProtoSyncError::EnvironmentVariableError(
                    TOKEN_ENCRYPTION_PUBLIC_KEY_ENV_VAR.to_string()
                )
            })?;

        let token_encryption_public_key =
            openssl::base64::decode_block(&token_encryption_public_key).map_err(|_| {
                FediProtoSyncError::EnvironmentVariableError(
                    TOKEN_ENCRYPTION_PUBLIC_KEY_ENV_VAR.to_string()
                )
            })?;

        let token_encryption_public_key =
            openssl::rsa::Rsa::public_key_from_pem(&token_encryption_public_key).map_err(|_| {
                FediProtoSyncError::EnvironmentVariableError(
                    TOKEN_ENCRYPTION_PUBLIC_KEY_ENV_VAR.to_string()
                )
            })?;

        // Read 'USER_AGENT' environment variable.
        let user_agent =
            std::env::var(USER_AGENT_ENV_VAR).unwrap_or_else(|_| "FediProto Sync".to_string());

        let user_agent = format!("{}/v{}", user_agent, GIT_VERSION);

        // Read 'MASTODON_SERVER' environment variable.
        let mastodon_server = std::env::var(MASTODON_SERVER_ENV_VAR).map_err(|_| {
            FediProtoSyncError::EnvironmentVariableError(MASTODON_SERVER_ENV_VAR.to_string())
        })?;

        // Read 'MASTODON_CLIENT_ID' environment variable.
        let mastodon_client_id = std::env::var(MASTODON_CLIENT_ID_ENV_VAR).map_err(|_| {
            FediProtoSyncError::EnvironmentVariableError(MASTODON_CLIENT_ID_ENV_VAR.to_string())
        })?;

        // Read 'MASTODON_CLIENT_SECRET' environment variable.
        let mastodon_client_secret =
            std::env::var(MASTODON_CLIENT_SECRET_ENV_VAR).map_err(|_| {
                FediProtoSyncError::EnvironmentVariableError(
                    MASTODON_CLIENT_SECRET_ENV_VAR.to_string()
                )
            })?;

        // Read 'MASTODON_REDIRECT_URI' environment variable.
        let mastodon_redirect_uri = std::env::var(MASTODON_REDIRECT_URI_ENV_VAR).map_err(|_| {
            FediProtoSyncError::EnvironmentVariableError(MASTODON_REDIRECT_URI_ENV_VAR.to_string())
        })?;

        // Read 'BLUESKY_PDS_SERVER' environment variable.
        let bluesky_pds_server = std::env::var(BLUESKY_PDS_SERVER_ENV_VAR).map_err(|_| {
            FediProtoSyncError::EnvironmentVariableError(BLUESKY_PDS_SERVER_ENV_VAR.to_string())
        })?;

        // Read 'BLUESKY_HANDLE' environment variable.
        let bluesky_handle = std::env::var(BLUESKY_HANDLE_ENV_VAR).map_err(|_| {
            FediProtoSyncError::EnvironmentVariableError(BLUESKY_HANDLE_ENV_VAR.to_string())
        })?;

        // Read 'BLUESKY_APP_PASSWORD' environment variable.
        let bluesky_app_password = std::env::var(BLUESKY_APP_PASSWORD_ENV_VAR).map_err(|_| {
            FediProtoSyncError::EnvironmentVariableError(BLUESKY_APP_PASSWORD_ENV_VAR.to_string())
        })?;

        // Read 'SYNC_INTERVAL_SECONDS' environment variable.
        let sync_interval = std::time::Duration::from_secs(
            std::env::var(SYNC_INTERVAL_SECONDS_ENV_VAR)
                .unwrap_or("300".to_string())
                .parse::<u64>()
                .map_err(|_| {
                    FediProtoSyncError::EnvironmentVariableParseError(
                        SYNC_INTERVAL_SECONDS_ENV_VAR.to_string()
                    )
                })?
        );

        // Read 'BLUESKY_VIDEO_ALWAYS_FALLBACK' environment variable.
        let bluesky_video_always_fallback = std::env::var(BLUESKY_VIDEO_ALWAYS_FALLBACK_ENV_VAR)
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .map_err(|_| {
                FediProtoSyncError::EnvironmentVariableParseError(
                    BLUESKY_VIDEO_ALWAYS_FALLBACK_ENV_VAR.to_string()
                )
            })?;

        Ok(Self {
            mode,
            auth_server_address,
            auth_server_port,
            database_type,
            database_url,
            token_encryption_private_key,
            token_encryption_public_key,
            user_agent,
            mastodon_server,
            mastodon_client_id,
            mastodon_client_secret,
            mastodon_redirect_uri,
            bluesky_pds_server,
            bluesky_handle,
            bluesky_app_password,
            sync_interval,
            bluesky_video_always_fallback
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FediProtoSyncMode {
    Auth,
    Normal
}

impl From<&str> for FediProtoSyncMode {
    fn from(s: &str) -> Self {
        match s {
            "auth" => FediProtoSyncMode::Auth,
            "normal" => FediProtoSyncMode::Normal,
            _ => FediProtoSyncMode::Normal
        }
    }
}

impl std::str::FromStr for FediProtoSyncMode {
    type Err = FediProtoSyncError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auth" => Ok(FediProtoSyncMode::Auth),
            "normal" => Ok(FediProtoSyncMode::Normal),
            _ => Err(FediProtoSyncError::EnvironmentVariableParseError(
                FEDIPROTO_SYNC_MODE_ENV_VAR.to_string()
            ))
        }
    }
}

/// The type of database to use.
#[derive(Debug, Clone)]
pub enum DatabaseType {
    /// PostgreSQL database
    Postgres,

    /// SQLite database
    SQLite
}

impl From<&str> for DatabaseType {
    fn from(s: &str) -> Self {
        match s {
            "Postgres" => DatabaseType::Postgres,
            "SQLite" => DatabaseType::SQLite,
            _ => DatabaseType::Postgres
        }
    }
}

impl std::str::FromStr for DatabaseType {
    type Err = FediProtoSyncError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Postgres" => Ok(DatabaseType::Postgres),
            "SQLite" => Ok(DatabaseType::SQLite),
            _ => Err(FediProtoSyncError::InvalidDatabaseType)
        }
    }
}
