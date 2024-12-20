use crate::{
    api_calls::{AddApiAuth, ApiAuthConfig, ApiError},
    types::app_bsky
};

/// Get status details for a video processing job.
/// 
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
/// 
/// ## Arguments
/// 
/// * `host_name` - The hostname of the server to connect to.
/// * `api_auth_config` - The authentication configuration to use.
/// * `job_id` - The ID of the job to get the status of.
pub async fn get_job_status(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    job_id: &str
) -> Result<app_bsky::video::GetJobStatusResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.video.getJobStatus", host_name);

    let query_params = vec![("jobId", job_id)];

    
    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: app_bsky::video::GetJobStatusResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Get video upload limits for the authenticated user.
/// 
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
/// 
/// ## Arguments
/// 
/// * `host_name` - The hostname of the server to connect to.
/// * `api_auth_config` - The authentication configuration to use.
pub async fn get_upload_limits(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig
) -> Result<app_bsky::video::GetUploadLimitsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.video.getUploadLimits", host_name);

    
    let response = client
        .get(&api_url)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: app_bsky::video::GetUploadLimitsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Upload a video to be processed then stored on the PDS.
/// 
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
/// 
/// ## Arguments
/// 
/// * `host_name` - The hostname of the server to connect to.
/// * `api_auth_config` - The authentication configuration to use.
/// * `video` - The video to upload.
pub async fn upload_video<T: Into<reqwest::Body>>
(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    video: T,
    did: &str,
    name: &str
) -> Result<crate::types::app_bsky::video::JobStatus, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.video.uploadVideo", host_name);

    
    let response = client
        .post(&api_url)
        .add_api_auth(api_auth_config.clone())
        .query(&[("did", did), ("name", name)])
        .header("Content-Type", "video/mp4")
        .body(video)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK | reqwest::StatusCode::CONFLICT => {
            let response_body: crate::types::app_bsky::video::JobStatus = response.json().await?;

            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}
