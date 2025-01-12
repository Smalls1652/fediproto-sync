use anyhow::Context;
use async_session::{MemoryStore, Session, SessionStore};
use axum::{
    extract::{Query, State},
    http::{HeaderMap, header::SET_COOKIE},
    response::{Html, IntoResponse, Redirect}
};
use axum_extra::{TypedHeader, headers};
use fediproto_sync_db::{models::NewCachedServiceToken, operations::insert_cached_service_token};
use oauth2::{
    AuthorizationCode,
    CsrfToken,
    PkceCodeVerifier,
    TokenResponse,
    reqwest::async_http_client
};

use crate::{
    FediProtoSyncWebServerAppState,
    error::FediProtoSyncWebError,
    web::{AuthRequest, check_for_existing_token}
};

static AUTH_SESSION: &str = "MASTODON_AUTH_SESSION";
static CSRF_TOKEN: &str = "csrf_token";
static PKCE_VERIFIER_SECRET: &str = "pkce_verifier_secret";

/// The OAuth2 "login" endpoint for Mastodon.
///
/// ## Arguments
///
/// * `app_state` - The application state.
pub async fn login_endpoint(
    State(app_state): State<FediProtoSyncWebServerAppState>
) -> Result<impl IntoResponse, FediProtoSyncWebError> {
    // Get a database connection from the pool and check if a token already exists.
    let db_connection = &mut app_state
        .db_pool
        .get()
        .context("Failed to get the database connection.")?;

    let token_exists = check_for_existing_token(db_connection, "mastodon")?;

    if token_exists {
        let headers = HeaderMap::new();

        return Ok((headers, Redirect::to("/auth/mastodon/already_authorized")));
    }

    // Get the Mastodon OAuth client and the memory store.
    let mastodon_client = app_state.mastodon_oauth_client.clone();
    let memory_store = app_state.memory_store.clone();

    // Generate the authorization URL, CSRF token, and PKCE code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = mastodon_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("read:statuses".to_string()))
        .add_scope(oauth2::Scope::new("profile".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    // Insert the CSRF token and PKCE verifier secret into the session.
    let mut session = Session::new();
    session
        .insert(CSRF_TOKEN, &csrf_token)
        .context("Failed to insert CSRF token into the session.")?;

    session
        .insert(PKCE_VERIFIER_SECRET, &pkce_code_verifier.secret())
        .context("Failed to insert PKCE verifier secret into the session.")?;

    // Store the session in the memory store.
    let cookie = memory_store
        .store_session(session)
        .await
        .context("Failed to store the session.")?
        .context("An unexpected error occurred while retrieving the session.")?;

    // Set the session cookie.
    let cookie = format!("{AUTH_SESSION}={cookie}; SameSite=Lax; HttpOnly; Secure; Path=/");

    // Redirect to the authorization URL.
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("Failed to parse the cookie.")?
    );

    Ok((headers, Redirect::to(auth_url.as_ref())))
}

/// Validate the CSRF token.
///
/// ## Arguments
///
/// * `auth_request` - The authorization request.
/// * `cookies` - The cookies.
/// * `memory_store` - The memory store.
async fn validate_csrf(
    auth_request: &AuthRequest,
    cookies: &headers::Cookie,
    memory_store: &MemoryStore
) -> Result<PkceCodeVerifier, FediProtoSyncWebError> {
    tracing::info!("Mastodon auth: Validating the CSRF token...");

    // Get the session cookie.
    let cookie = cookies
        .get(AUTH_SESSION)
        .context("Failed to get the session cookie.")?
        .to_string();

    // Load the session from the memory store.
    let session = match memory_store
        .load_session(cookie)
        .await
        .context("Failed to load the session.")?
    {
        Some(session) => session,
        None => return Err(anyhow::anyhow!("Session not found.").into())
    };

    // Get the CSRF token and PKCE verifier secret from the session.
    let csrf_token = session
        .get::<CsrfToken>(CSRF_TOKEN)
        .context("Failed to get the CSRF token from the session.")?
        .to_owned();

    let pkce_verifier_secret = session
        .get::<String>(PKCE_VERIFIER_SECRET)
        .context("Failed to get the PKCE verifier secret from the session.")?
        .to_owned();

    let pkce_verifier = PkceCodeVerifier::new(pkce_verifier_secret);

    // Destroy the session.
    memory_store
        .destroy_session(session)
        .await
        .context("Failed to destroy the session.")?;

    // Check if the CSRF token matches the state.
    if *csrf_token.secret() != auth_request.state {
        tracing::error!("Mastodon auth: CSRF token mismatch.");
        return Err((anyhow::anyhow!("CSRF token mismatch.")).into());
    }

    tracing::info!("Mastodon auth: CSRF token validated.");

    Ok(pkce_verifier)
}

/// The OAuth2 "authorized" endpoint for Mastodon.
///
/// ## Arguments
///
/// * `query` - The query.
/// * `app_state` - The application state.
/// * `cookies` - The cookies.
pub async fn authorized_endpoint(
    Query(query): Query<AuthRequest>,
    State(app_state): State<FediProtoSyncWebServerAppState>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>
) -> Result<impl IntoResponse, FediProtoSyncWebError> {
    // Validate the CSRF token and get the PKCE verifier.
    let pkce_verifier = validate_csrf(&query, &cookies, &app_state.memory_store).await?;

    let mastodon_client = app_state.mastodon_oauth_client.clone();
    let config = app_state.config.clone();

    // Exchange the code for the token.
    let token = mastodon_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .context("Failed to exchange the code for the token.")?;

    // Get a database connection from the pool.
    let db_connection = &mut app_state
        .db_pool
        .get()
        .context("Failed to get the database connection.")?;

    // Get the token encryption public key.
    let encryption_public_key = &config.token_encryption_public_key.clone();

    // Get the access token.
    let access_token = token.access_token().secret().to_string();

    // Get the scopes.
    let scopes = token.scopes();
    let scopes = match scopes {
        Some(scopes) => Some(
            scopes
                .iter()
                .map(|scope| scope.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        ),
        None => None
    };

    // Create a new token and insert it into the database.
    let new_token = NewCachedServiceToken::new(
        encryption_public_key,
        "mastodon",
        &access_token,
        None,
        None,
        scopes
    );

    let new_token = match new_token {
        Ok(token) => token,
        Err(e) => return Err(anyhow::anyhow!("Failed to create new token: {:?}", e).into())
    };

    let insert_result = insert_cached_service_token(db_connection, &new_token);

    match insert_result {
        Ok(_) => (),
        Err(e) => {
            return Err(
                anyhow::anyhow!("Failed to insert the token into the database: {:?}", e).into()
            );
        }
    }

    tracing::info!("Mastodon auth: Authorized!");

    Ok("Authorized!")
}

/// The OAuth2 "already authorized" endpoint for Mastodon.
pub async fn already_authorized_endpoint() -> Result<impl IntoResponse, FediProtoSyncWebError> {
    Ok(((), Html("Mastodon already authorized!")))
}
