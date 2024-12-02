
#[derive(Debug)]
#[allow(dead_code)]
pub struct Error {
    pub message: String,

    pub kind: ErrorKind,

    pub source: Option<Box<dyn std::error::Error>>
}

#[allow(dead_code)]
impl Error {
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

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ErrorKind {
    EnvironmentVariableError,
    DatabaseConnectionError
}

impl std::fmt::Display for ErrorKind {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter
    ) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
