use super::api_responses::{ActorFeedsResponse, ActorLikesResponse, AuthorFeedResponse};
use crate::api_calls::{AddApiAuth, ApiAuthConfig, ApiError};

/// Get a list of feeds (feed generator records) created by the actor (in the
/// actor's repo).
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `actor` - Handle or DID of the account to fetch the feeds of.
/// * `limit` - The maximum number of feeds to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
pub async fn get_actor_feeds(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<ActorFeedsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.feed.getActorFeeds", host_name);

    let mut query_params = Vec::new();
    query_params.push(("actor", actor));

    let limit_string = limit.unwrap_or_else(|| 50).to_string();
    query_params.push(("limit", limit_string.as_str()));

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
            let response_body: ActorFeedsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Get a list of posts liked by an actor. Requires auth, actor must be the
/// requesting account.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `actor` - Handle or DID of the account to fetch the likes of.
/// * `limit` - The maximum number of likes to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
pub async fn get_actor_likes(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<ActorLikesResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.feed.getActorLikes", host_name);

    let mut query_params = Vec::new();
    query_params.push(("actor", actor));

    let limit_string = limit.unwrap_or_else(|| 50).to_string();
    query_params.push(("limit", limit_string.as_str()));

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
            let response_body: ActorLikesResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Get a view of an actor's 'author feed' (post and reposts by the author).
/// Does not require auth.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `actor` - Handle or DID of the account to fetch the author feed of.
/// * `limit` - The maximum number of posts to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
/// * `filter` - A filter for the feed.
/// * `include_pins` - Whether to include pinned posts in the feed.
pub async fn get_author_feed(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>,
    filter: Option<&str>,
    include_pins: bool
) -> Result<AuthorFeedResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.feed.getAuthorFeed", host_name);

    let mut query_params = Vec::new();
    query_params.push(("actor", actor));

    let limit_string = limit.unwrap_or_else(|| 50).to_string();
    query_params.push(("limit", limit_string.as_str()));

    if let Some(cursor) = cursor {
        query_params.push(("cursor", cursor));
    }

    if let Some(filter) = filter {
        query_params.push(("filter", filter));
    }

    let include_pins_string = include_pins.to_string();
    query_params.push(("includePins", include_pins_string.as_str()));

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: AuthorFeedResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}
