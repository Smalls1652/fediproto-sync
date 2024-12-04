
/// Error value for FediProtoSync.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Error {
    /// A message describing the error.
    pub message: String,

    /// The kind of error that occurred.
    pub kind: ErrorKind,

    /// The source of the error, if any.
    pub source: Option<Box<dyn std::error::Error>>
}

#[allow(dead_code)]
impl Error {
    /// Create a new error.
    /// 
    /// ## Arguments
    /// 
    /// - `message` - A message describing the error.
    /// - `kind` - The kind of error that occurred.
    pub fn new(
        message: &str,
        kind: ErrorKind
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
    /// - `message` - A message describing the error.
    /// - `kind` - The kind of error that occurred.
    /// - `source` - The source of the error.
    pub fn with_source(
        message: &str,
        kind: ErrorKind,
        source: Box<dyn std::error::Error>
    ) -> Self {
        Self {
            message: message.to_string(),
            kind,
            source: Some(source)
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter
    ) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| &**e)
    }
}

/// The kind of error that occurred.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ErrorKind {
    /// An error occurred while trying to read an environment variable.
    EnvironmentVariableError,

    /// An error occurred while trying to authenticate.
    AuthenticationError,

    /// An error occurred while trying to connect to the database.
    DatabaseConnectionError,

    /// An error occurred while uploading a video to BlueSky.
    VideoUploadError
}

impl std::fmt::Display for ErrorKind {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter
    ) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
