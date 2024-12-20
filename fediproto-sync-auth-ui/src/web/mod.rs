/// Mastodon authentication endpoints.
pub mod mastodon_auth;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use fediproto_sync_db::AnyConnection;
use serde::Deserialize;

use crate::error::FediProtoSyncWebError;

/// Represents the response from the Mastodon OAuth2 token endpoint.
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    /// The OAuth2 code.
    pub code: String,

    /// The OAuth2 state.
    pub state: String
}

/// Check if a token already exists for the given service name.
/// 
/// ## Arguments
/// 
/// * `db_connection` - The database connection.
/// * `service_name` - The name of the service.
pub fn check_for_existing_token(
    db_connection: &mut PooledConnection<ConnectionManager<AnyConnection>>,
    service_name: &str
) -> Result<bool, FediProtoSyncWebError> {
    let result = fediproto_sync_db::operations::get_cached_service_token_by_service_name(
        db_connection,
        service_name
    );

    match result {
        Ok(token) => {
            if token.is_some() {
                return Ok(true);
            } else {
                return Ok(false);
            }
        }
        Err(e) => return Err(anyhow::anyhow!("Failed to get the token: {:?}", e).into())
    }
}
