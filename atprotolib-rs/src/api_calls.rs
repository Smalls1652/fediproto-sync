use reqwest::{RequestBuilder, Response, StatusCode};
use serde::{Deserialize, Serialize};

/// Represents API authentication configuration.
#[derive(Debug, Clone)]
pub struct ApiAuthConfig {
    /// The data for the API authentication configuration.
    pub data: ApiAuthConfigData
}

/// Represents the data for API authentication configuration.
#[derive(Debug, Clone)]
pub enum ApiAuthConfigData {
    /// No authentication.
    None,

    /// Admin user authentication.
    AdminUser(ApiAuthAdminUser),

    /// Bearer token authentication.
    BearerToken(ApiAuthBearerToken)
}

/// Represents an admin user for API authentication.
#[derive(Debug, Clone)]
pub struct ApiAuthAdminUser {
    /// The username of the admin user.
    pub username: String,

    /// The password of the admin user.
    pub password: String
}

/// Represents a bearer token for API authentication.
#[derive(Debug, Clone)]
pub struct ApiAuthBearerToken {
    /// The bearer token.
    pub token: String
}

/// A trait to add ATProto API authentication to a `RequestBuilder` instance from `reqwest`.
pub trait AddApiAuth {
    /// Adds ATProto API authentication to a `RequestBuilder` instance.
    /// 
    /// ## Arguments
    /// 
    /// * `api_auth_config` - The API authentication configuration.
    fn add_api_auth(
        self,
        api_auth_config: ApiAuthConfig
    ) -> Self;
}

impl AddApiAuth for RequestBuilder {
    /// Adds ATProto API authentication to a `RequestBuilder` instance.
    /// 
    /// ## Arguments
    /// 
    /// * `api_auth_config` - The API authentication configuration.
    fn add_api_auth(
        self,
        api_auth_config: ApiAuthConfig
    ) -> Self {
        match api_auth_config.data {
            // The API authentication configuration is for an admin user.
            ApiAuthConfigData::AdminUser(auth) => {
                self.basic_auth(auth.username, Some(auth.password))
            }

            // The API authentication configuration is for a bearer token.
            ApiAuthConfigData::BearerToken(auth) => self.bearer_auth(auth.token),

            // The API authentication configuration is for no authentication.
            ApiAuthConfigData::None => self
        }
    }
}

/// Represents an API error.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    /// The returned error code.
    #[serde(rename = "error")]
    pub error: String,

    /// The returned error message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// The kind of API error. Typically corresponds to the HTTP status code.
    #[serde(skip, default)]
    pub kind: ApiErrorKind
}

impl ApiError {
    /// Creates a new `ApiError` instance from a `Response`.
    /// 
    /// ## Arguments
    /// 
    /// * `response` - The `Response` instance.
    pub async fn new(response: Response) -> Result<Self, Box<dyn std::error::Error>> {
        let status = response.status();
        let text = response.text().await?;
        let error: ApiError = serde_json::from_str(&text)?;

        let kind = match status {
            StatusCode::BAD_REQUEST => ApiErrorKind::BadRequest,
            StatusCode::UNAUTHORIZED => ApiErrorKind::Unauthorized,
            StatusCode::FORBIDDEN => ApiErrorKind::Forbidden,
            StatusCode::NOT_FOUND => ApiErrorKind::NotFound,
            StatusCode::METHOD_NOT_ALLOWED => ApiErrorKind::MethodNotAllowed,
            StatusCode::CONFLICT => ApiErrorKind::Conflict,
            StatusCode::INTERNAL_SERVER_ERROR => ApiErrorKind::InternalServerError,
            StatusCode::SERVICE_UNAVAILABLE => ApiErrorKind::ServiceUnavailable,
            StatusCode::GATEWAY_TIMEOUT => ApiErrorKind::GatewayTimeout,
            _ => ApiErrorKind::Unknown
        };

        Ok(ApiError {
            error: error.error,
            message: error.message,
            kind
        })
    }
}

impl std::error::Error for ApiError {}

impl std::fmt::Display for ApiError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter
    ) -> std::fmt::Result {
        write!(f, "API Error: {} - {}", self.error, self.message.clone().unwrap_or_else(|| "N/A".to_string()))
    }
}

/// Represents the kind of an API error.
#[derive(Debug, Clone)]
pub enum ApiErrorKind {
    /// An unknown API error.
    Unknown,

    /// A bad request API error.
    BadRequest,

    /// An unauthorized API error.
    Unauthorized,

    /// A forbidden API error.
    Forbidden,

    /// A not found API error.
    NotFound,

    /// A method not allowed API error.
    MethodNotAllowed,

    /// A conflict API error.
    Conflict,

    /// An internal server error API error.
    InternalServerError,

    /// A service unavailable API error.
    ServiceUnavailable,

    /// A gateway timeout API error.
    GatewayTimeout
}

impl Default for ApiErrorKind {
    fn default() -> Self {
        ApiErrorKind::Unknown
    }
}

impl std::fmt::Display for ApiErrorKind {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter
    ) -> std::fmt::Result {
        match self {
            ApiErrorKind::Unknown => write!(f, "Unknown"),
            ApiErrorKind::BadRequest => write!(f, "Bad Request"),
            ApiErrorKind::Unauthorized => write!(f, "Unauthorized"),
            ApiErrorKind::Forbidden => write!(f, "Forbidden"),
            ApiErrorKind::NotFound => write!(f, "Not Found"),
            ApiErrorKind::MethodNotAllowed => write!(f, "Method Not Allowed"),
            ApiErrorKind::Conflict => write!(f, "Conflict"),
            ApiErrorKind::InternalServerError => write!(f, "Internal Server Error"),
            ApiErrorKind::ServiceUnavailable => write!(f, "Service Unavailable"),
            ApiErrorKind::GatewayTimeout => write!(f, "Gateway Timeout")
        }
    }
}
