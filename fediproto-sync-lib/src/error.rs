use thiserror::Error;

/// Error value for FediProtoSync.
#[derive(Error, Debug, Clone)]
#[allow(dead_code)]
pub enum FediProtoSyncError {
    /// An error occurred while trying to read an environment variable.
    #[error("Failed to read environment variable: {0}")]
    EnvironmentVariableError(String),

    /// An error occurred while trying to parse an environment variable.
    #[error("Failed to parse environment variable: {0}")]
    EnvironmentVariableParseError(String),

    /// An error occurred while trying to authenticate.
    #[error("Failed to authenticate to {0}.")]
    AuthenticationError(AuthenticationSource),

    /// An invalid database type was specified.
    #[error("Invalid database type.")]
    InvalidDatabaseType,

    /// An error occurred while encrypting a value.
    #[error("Failed to encrypt value.")]
    EncryptionError,

    /// An error occurred while decrypting a value.
    #[error("Failed to decrypt value.")]
    DecryptionError,

    /// An error occurred while creating a HTTP client.
    #[error("Failed to create HTTP client.")]
    HttpClientCreationError,

    /// An error occurred while uploading a video to BlueSky.
    #[error("Failed to upload video.")]
    VideoUploadError,

    /// An error occurred while removing a temporary file.
    #[error("Failed to remove temporary file.")]
    TempFileRemovalError,

    /// An error occurred while running the web server.
    #[error("An error occurred with the web server.")]
    WebServerError,

    /// An error occurred while trying to compress an image.
    #[error("Failed to compress image.")]
    ImageCompressionError
}

#[derive(Debug, Clone)]
pub enum AuthenticationSource {
    Mastodon,
    BlueSky
}

impl std::fmt::Display for AuthenticationSource {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        match self {
            AuthenticationSource::Mastodon => write!(f, "Mastodon"),
            AuthenticationSource::BlueSky => write!(f, "BlueSky")
        }
    }
}
