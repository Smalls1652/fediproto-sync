use super::{api_requests::CreateReportRequest, api_responses::CreateReportResponse};
use crate::api_calls::{AddApiAuth, ApiAuthConfig, ApiError};

/// Submit a moderation report regarding an atproto account or record.
/// Implemented by moderation services (with PDS proxying), and requires auth.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to send the request to.
/// * `api_auth_config` - The authentication configuration to use for the
///   request.
/// * `report` - The report to create.
pub async fn create_report(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    report: CreateReportRequest
) -> Result<CreateReportResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.moderation.createReport",
        host_name
    );

    let response = client
        .post(&api_url)
        .json(&report)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: CreateReportResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}
