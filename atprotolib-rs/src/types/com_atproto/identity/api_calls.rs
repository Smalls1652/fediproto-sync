use crate::{
    api_calls::{AddApiAuth, ApiAuthConfig, ApiError},
    types::com_atproto
};

/// Describe the credentials that should be included in the DID doc of an account that is migrating to this service.
/// 
/// ## Arguments
/// 
/// * `host_name` - The host name of the service.
/// * `api_auth_config` - The API authentication configuration.
pub async fn get_recommended_did_credentials(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig
) -> Result<com_atproto::identity::GetRecommendedDidCredentialsResponse, Box<dyn std::error::Error>>
{
    let api_url = format!(
        "https://{}/xrpc/com.atproto.identity.getRecommendedDidCredentials",
        host_name
    );

    
    let response = client
        .get(&api_url)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: com_atproto::identity::GetRecommendedDidCredentialsResponse =
                response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Resolves a handle (domain name) to a DID.
/// 
/// ## Arguments
/// 
/// * `host_name` - The host name of the service.
/// * `api_auth_config` - The API authentication configuration.
/// * `handle` - The handle to resolve.
pub async fn resolve_handle(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    handle: &str
) -> Result<com_atproto::identity::ResolveHandleResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.identity.resolveHandle",
        host_name
    );

    let query_params = vec![("handle", handle)];

    
    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: com_atproto::identity::ResolveHandleResponse =
                response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Signs a PLC operation to update some value(s) in the requesting DID's document.
/// 
/// ## Arguments
/// 
/// * `host_name` - The host name of the service.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to sign the PLC operation.
pub async fn sign_plc_operation(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: com_atproto::identity::SignPlcOperationRequest
) -> Result<com_atproto::identity::SignPlcOperationResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.identity.signPlcOperation",
        host_name
    );

    
    let response = client
        .post(&api_url)
        .json(&request)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: com_atproto::identity::SignPlcOperationResponse =
                response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Validates a PLC operation to ensure that it doesn't violate a service's constraints or get the identity into a bad state, then submits it to the PLC registry
/// 
/// ## Arguments
/// 
/// * `host_name` - The host name of the service.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to submit the PLC operation.
pub async fn submit_plc_operation(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: com_atproto::identity::SubmitPlcOperationRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.identity.submitPlcOperation",
        host_name
    );

    
    let response = client
        .post(&api_url)
        .json(&request)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Updates the current account's handle. Verifies handle validity, and updates did:plc document if necessary. Implemented by PDS, and requires auth.
/// 
/// ## Arguments
/// 
/// * `host_name` - The host name of the service.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to update the handle.
pub async fn update_handle(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: com_atproto::identity::UpdateHandleRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.identity.updateHandle",
        host_name
    );

    
    let response = client
        .post(&api_url)
        .json(&request)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}
