use crate::GIT_VERSION;

/// The environment variable values for configuring the FediProtoSync
/// application.
#[derive(Debug, Clone)]
pub struct FediProtoSyncEnvVars {
    /// The mode to run the application in.
    pub mode: String,

    /// The type of database to use.
    pub database_type: DatabaseType,

    /// The URL/path to the database.
    pub database_url: String,

    /// The encryption key to use for token encryption.
    pub token_encryption_private_key: openssl::rsa::Rsa<openssl::pkey::Private>,

    /// The encryption IV to use for token encryption.
    pub token_encryption_public_key: openssl::rsa::Rsa<openssl::pkey::Public>,

    /// User-Agent string to use for HTTP requests.
    pub user_agent: String,

    /// The Mastodon server URL to connect to.
    pub mastodon_server: String,

    /// The client ID for the Mastodon application.
    pub mastodon_client_id: String,

    /// The client secret for the Mastodon application.
    pub mastodon_client_secret: String,

    /// The Mastodon access token to use for authentication.
    /// 
    /// ### ⚠️ Warning
    /// 
    /// **This property will be deprecated soon.**
    #[deprecated = "This property will be removed when OAuth2 is fully implemented."]
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
        // Read 'FEDIPROTO_SYNC_MODE' environment variable.
        let mode = std::env::var("FEDIPROTO_SYNC_MODE").unwrap_or("normal".to_string());

        // Read 'DATABASE_TYPE' environment variable.
        let database_type = std::env::var("DATABASE_TYPE")
            .unwrap_or("Postgres".to_string())
            .parse::<DatabaseType>()
            .map_err(|e| {
                crate::error::Error::with_source(
                    "Failed to parse the DATABASE_TYPE environment variable.",
                    crate::error::ErrorKind::EnvironmentVariableError,
                    Box::new(e)
                )
            })?;

        // Read 'DATABASE_URL' environment variable.
        let database_url = std::env::var("DATABASE_URL").map_err(|e| {
            crate::error::Error::with_source(
                "Failed to read DATABASE_URL environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;

        // Read 'TOKEN_ENCRYPTION_PRIVATE_KEY' environment variable.
        let token_encryption_private_key = std::env::var("TOKEN_ENCRYPTION_PRIVATE_KEY")
            .map_err(|e| {
            crate::error::Error::with_source(
                "Failed to read TOKEN_ENCRYPTION_PRIVATE_KEY environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;
        let token_encryption_private_key = openssl::base64::decode_block(&token_encryption_private_key)
            .map_err(|e| {
            crate::error::Error::with_source(
                "Failed to decode the TOKEN_ENCRYPTION_PRIVATE_KEY environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;
        let token_encryption_private_key = openssl::rsa::Rsa::private_key_from_pem(&token_encryption_private_key)
            .map_err(|e| {
            crate::error::Error::with_source(
                "Failed to decode TOKEN_ENCRYPTION_PRIVATE_KEY environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;

        // Read 'TOKEN_ENCRYPTION_PUBLIC_KEY' environment variable.
        let token_encryption_public_key = std::env::var("TOKEN_ENCRYPTION_PUBLIC_KEY")
            .map_err(|e| {
            crate::error::Error::with_source(
                "Failed to read TOKEN_ENCRYPTION_PUBLIC_KEY environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;
        let token_encryption_public_key = openssl::base64::decode_block(&token_encryption_public_key)
            .map_err(|e| {
            crate::error::Error::with_source(
                "Failed to decode the TOKEN_ENCRYPTION_PUBLIC_KEY environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;
        let token_encryption_public_key = openssl::rsa::Rsa::public_key_from_pem(&token_encryption_public_key)
            .map_err(|e| {
            crate::error::Error::with_source(
                "Failed to decode TOKEN_ENCRYPTION_PUBLIC_KEY environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;

        // Read 'USER_AGENT' environment variable.
        let user_agent =
            std::env::var("USER_AGENT").unwrap_or_else(|_| "FediProtoSync".to_string());

        let user_agent = format!("{}/v{}", user_agent, GIT_VERSION);

        // Read 'MASTODON_SERVER' environment variable.
        let mastodon_server = std::env::var("MASTODON_SERVER").map_err(|e| {
            crate::error::Error::with_source(
                "Failed to read MASTODON_SERVER environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;

        // Read 'MASTODON_CLIENT_ID' environment variable.
        let mastodon_client_id = std::env::var("MASTODON_CLIENT_ID").map_err(|e| {
            crate::error::Error::with_source(
                "Failed to read MASTODON_CLIENT_ID environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;

        // Read 'MASTODON_CLIENT_SECRET' environment variable.
        let mastodon_client_secret = std::env::var("MASTODON_CLIENT_SECRET").map_err(|e| {
            crate::error::Error::with_source(
                "Failed to read MASTODON_CLIENT_SECRET environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;

        // Read 'MASTODON_ACCESS_TOKEN' environment variable.
        let mastodon_access_token = std::env::var("MASTODON_ACCESS_TOKEN").map_err(|e| {
            crate::error::Error::with_source(
                "Failed to read MASTODON_ACCESS_TOKEN environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;

        // Read 'BLUESKY_PDS_SERVER' environment variable.
        let bluesky_pds_server = std::env::var("BLUESKY_PDS_SERVER").map_err(|e| {
            crate::error::Error::with_source(
                "Failed to read BLUESKY_PDS_SERVER environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;

        // Read 'BLUESKY_HANDLE' environment variable.
        let bluesky_handle = std::env::var("BLUESKY_HANDLE").map_err(|e| {
            crate::error::Error::with_source(
                "Failed to read BLUESKY_HANDLE environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;

        // Read 'BLUESKY_APP_PASSWORD' environment variable.
        let bluesky_app_password = std::env::var("BLUESKY_APP_PASSWORD").map_err(|e| {
            crate::error::Error::with_source(
                "Failed to read BLUESKY_APP_PASSWORD environment variable.",
                crate::error::ErrorKind::EnvironmentVariableError,
                Box::new(e)
            )
        })?;

        // Read 'SYNC_INTERVAL_SECONDS' environment variable.
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

        // Read 'BLUESKY_VIDEO_ALWAYS_FALLBACK' environment variable.
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
            mode,
            database_type,
            database_url,
            token_encryption_private_key,
            token_encryption_public_key,
            user_agent,
            mastodon_server,
            mastodon_client_id,
            mastodon_client_secret,
            mastodon_access_token,
            bluesky_pds_server,
            bluesky_handle,
            bluesky_app_password,
            sync_interval,
            bluesky_video_always_fallback
        })
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
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Postgres" => Ok(DatabaseType::Postgres),
            "SQLite" => Ok(DatabaseType::SQLite),
            _ => Err(crate::error::Error::new(
                "Invalid database type.",
                crate::error::ErrorKind::EnvironmentVariableError
            ))
        }
    }
}
