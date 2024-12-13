/// Error value for FediProtoSync.
#[derive(Debug)]
#[allow(dead_code)]
pub struct FediProtoSyncError {
    /// A message describing the error.
    pub message: String,

    /// The kind of error that occurred.
    pub kind: FediProtoSyncErrorKind,

    /// The source of the error, if any.
    pub source: Option<Box<dyn std::error::Error>>
}

#[allow(dead_code)]
impl FediProtoSyncError {
    /// Create a new error.
    ///
    /// ## Arguments
    ///
    /// * `message` - A message describing the error.
    /// * `kind` - The kind of error that occurred.
    pub fn new(
        message: &str,
        kind: FediProtoSyncErrorKind
    ) -> Self {
        Self {
            message: message.to_string(),
            kind,
            source: None
        }
    }

    /// Create a new error with a source.
    ///
    /// ## Arguments
    ///
    /// * `message` - A message describing the error.
    /// * `kind` - The kind of error that occurred.
    /// * `source` - The source of the error.
    pub fn with_source(
        message: &str,
        kind: FediProtoSyncErrorKind,
        source: Box<dyn std::error::Error>
    ) -> Self {
        Self {
            message: message.to_string(),
            kind,
            source: Some(source)
        }
    }
}

impl std::fmt::Display for FediProtoSyncError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter
    ) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FediProtoSyncError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| &**e)
    }
}

/// The kind of error that occurred.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum FediProtoSyncErrorKind {
    /// An error occurred while trying to read an environment variable.
    EnvironmentVariableError,

    /// An error occurred while trying to authenticate.
    AuthenticationError,

    /// An error occurred while trying to connect to the database.
    DatabaseConnectionError,

    /// An error occurred while trying to run a database migration.
    DatabaseMigrationError,

    /// An invalid database type was specified.
    InvalidDatabaseType,

    /// An error occurred while querying the database.
    DatabaseQueryError,

    /// An error occurred while trying to insert a new record into the database.
    DatabaseInsertError,

    /// An error occurred while trying to delete a database record.
    DatabaseDeleteError,

    /// An error occurred while encrypting a value.
    EncryptionError,

    /// An error occurred while decrypting a value.
    DecryptionError,

    /// An error occurred while creating a HTTP client.
    HttpClientCreationError,

    /// An error occurred while uploading a video to BlueSky.
    VideoUploadError,

    /// An error occurred while removing a temporary file.
    TempFileRemovalError
}

impl std::fmt::Display for FediProtoSyncErrorKind {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter
    ) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
