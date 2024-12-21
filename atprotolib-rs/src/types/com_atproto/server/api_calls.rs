use super::{
    api_requests::{
        ConfirmEmailRequest,
        CreateAccountRequest,
        CreateAppPasswordRequest,
        CreateSessionRequest,
        DeactivateAccountRequest,
        DeleteAccountRequest,
        InviteCodeRequest,
        RequestPasswordResetRequest,
        ReserveSigningKeyRequest,
        ResetPasswordRequest,
        RevokeAppPasswordRequest,
        UpdateEmailRequest
    },
    api_responses::{
        CreateAccountResponse,
        CreateAppPasswordResponse,
        CreateSessionResponse,
        DescribeServerResponse,
        InviteCodeResponse,
        ListAppPasswordsResponse,
        RequestEmailUpdateResponse,
        ReserveSigningKeyResponse,
        ServiceAuthResponse,
        SessionResponse
    }
};
use crate::api_calls::{AddApiAuth, ApiAuthConfig, ApiError};

/// Confirm an email using a token from
/// `com.atproto.server.requestEmailConfirmation`.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to confirm the email.
pub async fn confirm_email(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: ConfirmEmailRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.server.confirmEmail", host_name);

    let request_builder = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request);

    let response = request_builder.send().await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Create an account. Implemented by PDS.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to create the account.
pub async fn create_account(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: CreateAccountRequest
) -> Result<CreateAccountResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.createAccount",
        host_name
    );

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: CreateAccountResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Create an App Password.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to create the App Password.
pub async fn create_app_password(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: CreateAppPasswordRequest
) -> Result<CreateAppPasswordResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.createAppPassword",
        host_name
    );

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: CreateAppPasswordResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Create an invite code.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to create the invite code.
pub async fn create_invite_code(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: InviteCodeRequest
) -> Result<InviteCodeResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.createInviteCode",
        host_name
    );

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: InviteCodeResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Create an authentication session.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to create the session.
pub async fn create_session(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: CreateSessionRequest
) -> Result<CreateSessionResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.createSession",
        host_name
    );

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: CreateSessionResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Deactivates a currently active account. Stops serving of repo, and future
/// writes to repo until reactivated. Used to finalize account migration with
/// the old host after the account has been activated on the new host.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to deactivate the account.
pub async fn deactivate_account(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: DeactivateAccountRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.deactivateAccount",
        host_name
    );

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Delete an actor's account with a token and password. Can only be called
/// after requesting a deletion token. Requires auth.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to delete the account.
pub async fn delete_account(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: DeleteAccountRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.deleteAccount",
        host_name
    );

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Describes the server's account creation requirements and capabilities.
/// Implemented by PDS.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
pub async fn describe_server(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig
) -> Result<DescribeServerResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.describeServer",
        host_name
    );

    let response = client
        .get(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: DescribeServerResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Get a signed token on behalf of the requesting DID for the requested
/// service.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `did` - The DID of the service that the token will be used to authenticate
///   with
/// * `expiry` - The time in Unix Epoch seconds that the JWT expires. Defaults
///   to 60 seconds in the future. The service may enforce certain time bounds
///   on tokens depending on the requested scope.
/// * `lexicon` - Lexicon (XRPC) method to bind the requested token to
pub async fn get_service_auth(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    did: &str,
    expiry: i64,
    lexicon: Option<&str>
) -> Result<ServiceAuthResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.getServiceAuth",
        host_name
    );

    let mut query_params = Vec::new();
    query_params.push(("aud", did));

    let expiry_str = expiry.to_string();
    query_params.push(("exp", &expiry_str));

    if let Some(lexicon) = lexicon {
        query_params.push(("lxm", lexicon));
    }

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match response.error_for_status() {
        Ok(response) => {
            let response_body: ServiceAuthResponse = response.json().await?;
            Ok(response_body)
        }
        Err(err) => Err(Box::new(err))
    }
}

/// Get information about the current auth session. Requires auth.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
pub async fn get_session(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig
) -> Result<SessionResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.server.getSession", host_name);

    let response = client
        .get(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: SessionResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// List all App Passwords.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
pub async fn list_app_passwords(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig
) -> Result<ListAppPasswordsResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.listAppPasswords",
        host_name
    );

    let response = client
        .get(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: ListAppPasswordsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Refresh an authentication session. Requires auth using the 'refreshJwt' (not
/// the 'accessJwt').
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
pub async fn refresh_session(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig
) -> Result<CreateSessionResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.refreshSession",
        host_name
    );

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: CreateSessionResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Request a token in order to update email.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `email` - The new email address.
pub async fn request_email_update(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    email: String
) -> Result<RequestEmailUpdateResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.requestEmailUpdate",
        host_name
    );

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&email)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: RequestEmailUpdateResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Initiate a user account password reset via email.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to reset the password.
pub async fn request_password_reset(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: RequestPasswordResetRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.requestPasswordReset",
        host_name
    );

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Reserve a repo signing key, for use with account creation. Necessary so that
/// a DID PLC update operation can be constructed during an account migraiton.
/// Public and does not require auth; implemented by PDS. NOTE: this endpoint
/// may change when full account migration is implemented.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to reserve the signing key.
pub async fn reserve_signing_key(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: ReserveSigningKeyRequest
) -> Result<ReserveSigningKeyResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.reserveSigningKey",
        host_name
    );

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: ReserveSigningKeyResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Reset a user account password using a token.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to reset the password.
pub async fn reset_password(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: ResetPasswordRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.resetPassword",
        host_name
    );

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Revoke an App Password by name.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to revoke the App Password.
pub async fn revoke_app_password(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: RevokeAppPasswordRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.server.revokeAppPassword",
        host_name
    );

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Update an account's email.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to update the email.
pub async fn update_email(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: UpdateEmailRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.server.updateEmail", host_name);

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}
