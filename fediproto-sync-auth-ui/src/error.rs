use axum::{
    http::StatusCode,
    response::{IntoResponse, Response}
};

/// Represents an error that occurred in the FediProtoSync web server.
#[derive(Debug)]
pub struct FediProtoSyncWebError(anyhow::Error);

impl IntoResponse for FediProtoSyncWebError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

impl<E> From<E> for FediProtoSyncWebError
where
    E: Into<anyhow::Error>
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
