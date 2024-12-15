use fediproto_sync_lib::{
    config::FediProtoSyncConfig,
    error::{FediProtoSyncError, FediProtoSyncErrorKind}
};
use oauth2::{basic::BasicClient, reqwest::async_http_client};

pub async fn get_mastodon_oauth_token(
    config: &FediProtoSyncConfig
) -> Result<
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    FediProtoSyncError
> {
    let client_id = oauth2::ClientId::new(config.mastodon_client_id.clone().unwrap());
    let client_secret = Some(oauth2::ClientSecret::new(
        config.mastodon_client_secret.clone().unwrap()
    ));

    let auth_url = oauth2::AuthUrl::new(format!(
        "https://{}/oauth/authorize",
        config.mastodon_server.clone()
    ))
    .map_err(|e| {
        FediProtoSyncError::with_source(
            "Failed to create Mastodon auth URL.",
            FediProtoSyncErrorKind::AuthenticationError,
            Box::new(e)
        )
    })?;

    let token_url = Some(
        oauth2::TokenUrl::new(format!(
            "https://{}/oauth/token",
            config.mastodon_server.clone()
        ))
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to create Mastodon token URL.",
                FediProtoSyncErrorKind::AuthenticationError,
                Box::new(e)
            )
        })?
    );

    let redirect_url =
        oauth2::RedirectUrl::new("urn:ietf:wg:oauth:2.0:oob".to_string()).map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to create Mastodon redirect URL.",
                FediProtoSyncErrorKind::AuthenticationError,
                Box::new(e)
            )
        })?;

    let client = BasicClient::new(client_id, client_secret, auth_url, token_url)
        .set_redirect_uri(redirect_url);

    let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

    let (auth_url, _csrf_token) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("read:statuses".to_string()))
        .add_scope(oauth2::Scope::new("profile".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    tracing::info!("Open this URL in your browser: {}", auth_url);

    tracing::info!("Enter the code from the browser:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let token_result = client
        .exchange_code(oauth2::AuthorizationCode::new(input.trim().to_string()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to exchange Mastodon auth code for token.",
                FediProtoSyncErrorKind::AuthenticationError,
                Box::new(e)
            )
        })?;

    Ok(token_result)
}
