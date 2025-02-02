use fediproto_sync_lib::{
    config::FediProtoSyncConfig,
    error::{AuthenticationSource, FediProtoSyncError}
};
use oauth2::basic::BasicClient;

/// Get the Mastodon OAuth2 client.
///
/// ## Arguments
///
/// * `config` - The FediProtoSync configuration.
/// * `redirect_uri` - The redirect URI.
pub fn get_mastodon_oauth_client(
    config: &FediProtoSyncConfig,
    redirect_uri: &str
) -> Result<BasicClient, FediProtoSyncError> {
    let client_id = oauth2::ClientId::new(config.mastodon_client_id.clone());
    let client_secret = Some(oauth2::ClientSecret::new(
        config.mastodon_client_secret.clone()
    ));

    let auth_url = oauth2::AuthUrl::new(format!(
        "https://{}/oauth/authorize",
        config.mastodon_server.clone()
    ))
    .map_err(|_| FediProtoSyncError::AuthenticationError(AuthenticationSource::Mastodon))?;

    let token_url = Some(
        oauth2::TokenUrl::new(format!(
            "https://{}/oauth/token",
            config.mastodon_server.clone()
        ))
        .map_err(|_| FediProtoSyncError::AuthenticationError(AuthenticationSource::Mastodon))?
    );

    let redirect_url = oauth2::RedirectUrl::new(redirect_uri.to_string())
        .map_err(|_| FediProtoSyncError::AuthenticationError(AuthenticationSource::Mastodon))?;

    let client = BasicClient::new(client_id, client_secret, auth_url, token_url)
        .set_redirect_uri(redirect_url);

    Ok(client)
}
