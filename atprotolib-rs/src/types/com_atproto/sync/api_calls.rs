use crate::api_calls::{AddApiAuth, ApiAuthConfig, ApiError};

use super::{api_requests::{CrawlRequest, NotifyOfUpdateRequest}, api_responses::{LatestCommitResponse, ListBlobsResponse, ListReposResponse, RepoStatusResponse}};

/// Get a blob associated with a given account. Returns the full blob as
/// originally uploaded. Does not require auth; implemented by PDS.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to send the request to.
/// * `api_auth_config` - The authentication configuration to use for the
///   request.
/// * `did` - The DID of the account.
/// * `cid` - The CID of the blob to get.
pub async fn get_blob(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    did: &str,
    cid: &str
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.getBlob", host_name);

    let query_params = vec![("did", did), ("cid", cid)];

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body = response.bytes().await?;
            Ok(response_body.to_vec())
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Get the current commit CID & revision of the specified repo. Does not
/// require auth.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to send the request to.
/// * `api_auth_config` - The authentication configuration to use for the
///   request.
/// * `did` - The DID of the repo.
pub async fn get_latest_commit(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    did: &str
) -> Result<LatestCommitResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/com.atproto.sync.getLatestCommit",
        host_name
    );

    let query_params = vec![("did", did)];

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: LatestCommitResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Get data blocks needed to prove the existence or non-existence of record in
/// the current version of repo. Does not require auth.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to send the request to.
/// * `api_auth_config` - The authentication configuration to use for the
///   request.
/// * `did` - The DID of the repo.
/// * `collection` - The collection of the record.
/// * `rkey` - The rkey of the record.
pub async fn get_record(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    did: &str,
    collection: &str,
    rkey: &str
) -> Result<String, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.getRecord", host_name);

    let query_params = vec![("did", did), ("collection", collection), ("rkey", rkey)];

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body = response.text().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Get the hosting status for a repository, on this server. Expected to be
/// implemented by PDS and Relay.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to send the request to.
/// * `api_auth_config` - The authentication configuration to use for the
///   request.
/// * `did` - The DID of the repo.
pub async fn get_repo_status(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    did: &str
) -> Result<RepoStatusResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.getRepoStatus", host_name);

    let query_params = vec![("did", did)];

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: RepoStatusResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Download a repository export as CAR file. Optionally only a 'diff' since a
/// previous revision. Does not require auth; implemented by PDS.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to send the request to.
/// * `api_auth_config` - The authentication configuration to use for the
///   request.
/// * `did` - The DID of the repo.
/// * `since` - The CID of the previous revision to diff against.
pub async fn get_repo(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    did: &str,
    since: Option<&str>
) -> Result<String, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.getRepo", host_name);

    let mut query_params = Vec::new();
    query_params.push(("did", did));

    if let Some(since) = since {
        query_params.push(("since", since));
    }

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body = response.text().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// List blob CIDs for an account, since some repo revision. Does not require
/// auth; implemented by PDS.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to send the request to.
/// * `api_auth_config` - The authentication configuration to use for the
///   request.
/// * `did` - The DID of the account.
/// * `since` - The CID of the previous revision to diff against.
/// * `limit` - The maximum number of blobs to return.
/// * `cursor` - The cursor to use for pagination.
pub async fn list_blobs(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    did: String,
    since: Option<String>,
    limit: Option<i32>,
    cursor: Option<String>
) -> Result<ListBlobsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.listBlobs", host_name);

    let mut query_params = Vec::new();
    query_params.push(("did", did));
    query_params.push(("limit", limit.unwrap_or_else(|| 500).to_string()));

    if let Some(since) = since {
        query_params.push(("since", since));
    }

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
            let response_body: ListBlobsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Enumerates all the DID, rev, and commit CID for all repos hosted by this
/// service. Does not require auth; implemented by PDS and Relay.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to send the request to.
/// * `api_auth_config` - The authentication configuration to use for the
///   request.
/// * `limit` - The maximum number of repos to return.
/// * `cursor` - The cursor to use for pagination.
pub async fn list_repos(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    limit: Option<i32>,
    cursor: Option<String>
) -> Result<ListReposResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.listRepos", host_name);

    let mut query_params = Vec::new();
    query_params.push(("limit", limit.unwrap_or_else(|| 100).to_string()));

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
            let response_body: ListReposResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Notify a crawling service of a recent update, and that crawling should
/// resume. Intended use is after a gap between repo stream events caused the
/// crawling service to disconnect. Does not require auth; implemented by Relay.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to send the request to.
/// * `api_auth_config` - The authentication configuration to use for the
///   request.
/// * `request` - The request to send.
pub async fn notify_of_update(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: NotifyOfUpdateRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.notifyOfUpdate", host_name);

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

/// Request a service to persistently crawl hosted repos. Expected use is new
/// PDS instances declaring their existence to Relays. Does not require auth.
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to send the request to.
/// * `api_auth_config` - The authentication configuration to use for the
///   request.
/// * `request` - The request to send.
pub async fn request_crawl(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: CrawlRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/com.atproto.sync.requestCrawl", host_name);

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
