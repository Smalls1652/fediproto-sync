use super::{
    api_requests::{
        AccountInfosRequest,
        DeleteAccountRequest,
        DisableAccountInvitesRequest,
        DisableInviteCodesRequest,
        EnableAccountInvitesRequest,
        SendEmailRequest,
        UpdateAccountEmailRequest,
        UpdateAccountHandleRequest,
        UpdateAccountPasswordRequest
    },
    api_responses::{
        AccountInfoResponse,
        AccountInfosResponse,
        InviteCodesResponse,
        SearchAccountsResponse,
        SendEmailResponse,
        SubjectStatusAccountResponse,
        SubjectStatusBlobResponse,
        SubjectStatusRecordResponse
    }
};
use crate::api_calls::{AddApiAuth, ApiAuthConfig, ApiError};

/// Delete a user account as an administrator.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to delete an account.
pub async fn delete_account(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: DeleteAccountRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.admin.deleteAccount", host_name);

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

/// Disable an account from receiving new invite codes, but does not invalidate
/// existing codes.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to disable account invites.
pub async fn disable_account_invites(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: DisableAccountInvitesRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.admin.disableAccountInvites",
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

/// Disable some set of codes and/or all codes associated with a set of users.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to disable invite codes.
pub async fn disable_invite_codes(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: DisableInviteCodesRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.admin.disableInviteCodes",
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

/// Re-enable an account's ability to receive invite codes.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to enable account invites.
pub async fn enable_account_invites(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: EnableAccountInvitesRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.admin.enableAccountInvites",
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

///  details about an account.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `did` - The DID of the account.
pub async fn get_account_info(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    did: String
) -> Result<AccountInfoResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.admin.getAccountInfo",
        host_name
    );

    let response = client
        .get(&api_url)
        .query(&[("did", did)])
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: AccountInfoResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

///  details about some accounts.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to get account infos.
pub async fn get_account_infos(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: AccountInfosRequest
) -> Result<AccountInfosResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.admin.getAccountInfos",
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
            let response_body: AccountInfosResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

///  an admin view of invite codes.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `limit` - The maximum number of invite codes to return. Defaults to 100.
/// * `cursor` - The cursor to use for pagination.
/// * `sort` - The sort order to use.
pub async fn get_invite_codes(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    limit: Option<i32>,
    cursor: Option<String>,
    sort: Option<String>
) -> Result<InviteCodesResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.admin.getInviteCodes",
        host_name
    );

    let mut query_params = Vec::new();
    query_params.push(("limit", limit.unwrap_or(100).to_string()));

    if let Some(cursor) = cursor {
        query_params.push(("cursor", cursor));
    }

    if let Some(sort) = sort {
        query_params.push(("sort", sort));
    }

    let response = client
        .post(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: InviteCodesResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// The subject to get the status of.
pub enum SubjectStatusSubject {
    /// The account DID.
    Account(String),

    /// The record URI.
    Record(String),

    /// The blob CID.
    Blob(String)
}

/// The response from getting the status of a subject.
pub enum SubjectStatusResponse {
    /// The account status.
    Account(SubjectStatusAccountResponse),

    /// The record status.
    Record(SubjectStatusRecordResponse),

    /// The blob status.
    Blob(SubjectStatusBlobResponse)
}

///  the service-specific admin status of a subject (account, record, or
/// blob).
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `subject` - The subject to get the status of.
pub async fn get_subject_status(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    subject: SubjectStatusSubject
) -> Result<SubjectStatusResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.admin.getSubjectStatus",
        host_name
    );

    let query_params = match &subject {
        SubjectStatusSubject::Account(did) => vec![("did", did)],
        SubjectStatusSubject::Record(uri) => vec![("uri", uri)],
        SubjectStatusSubject::Blob(blob) => vec![("blob", blob)]
    };

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => match &subject {
            SubjectStatusSubject::Account(_) => {
                let account_response: SubjectStatusAccountResponse = response.json().await?;
                Ok(SubjectStatusResponse::Account(account_response))
            }
            SubjectStatusSubject::Record(_) => {
                let record_response: SubjectStatusRecordResponse = response.json().await?;
                Ok(SubjectStatusResponse::Record(record_response))
            }
            SubjectStatusSubject::Blob(_) => {
                let blob_response: SubjectStatusBlobResponse = response.json().await?;
                Ok(SubjectStatusResponse::Blob(blob_response))
            }
        },
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

///  a list of accounts that matches your search query.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `query` - The search query.
/// * `limit` - The maximum number of accounts to return. Defaults to 10.
/// * `cursor` - The cursor to use for pagination.
pub async fn search_accounts(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    query: String,
    limit: Option<i32>,
    cursor: Option<String>
) -> Result<SearchAccountsResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.admin.searchAccounts",
        host_name
    );

    let mut query_params = Vec::new();
    query_params.push(("email", query));
    query_params.push(("limit", limit.unwrap_or_else(|| 10).to_string()));

    if let Some(cursor) = cursor {
        query_params.push(("cursor", cursor));
    }

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: SearchAccountsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Send an email to a user's account email address.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to send an email.
pub async fn send_email(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: SendEmailRequest
) -> Result<SendEmailResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.admin.sendEmail", host_name);

    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: SendEmailResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Administrative action to update an account's email.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to update an account's email.
pub async fn update_account_email(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: UpdateAccountEmailRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.admin.updateAccountEmail",
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

/// Administrative action to update an account's handle.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to update an account's handle.
pub async fn update_account_handle(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: UpdateAccountHandleRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.admin.updateAccountHandle",
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

/// Update the password for a user account as an administrator.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to update an account's password.
pub async fn update_account_password(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: UpdateAccountPasswordRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.admin.updateAccountPassword",
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
