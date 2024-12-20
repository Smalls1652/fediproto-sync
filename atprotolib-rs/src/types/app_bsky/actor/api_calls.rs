use crate::{
    api_calls::{AddApiAuth, ApiAuthConfig, ApiError},
    types::app_bsky
};

use super::{ProfileViewDetailed, SuggestionsResponse, SearchActorsTypeaheadResponse, SearchActorsResponse};

/// Get a detailed profile view of an actor. Does not require auth, but contains relevant metadata with auth.
/// 
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
/// 
/// ## Arguments
/// 
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `actor` - Handle or DID of the account to fetch the profile of.
pub async fn get_profile(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    actor: &str
) -> Result<ProfileViewDetailed, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.actor.getProfile", host_name);

    let query_params = vec![("actor", actor)];

    
    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: ProfileViewDetailed = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Get a list of suggested actors. Expected use is discovery of accounts to follow during new account onboarding.
/// 
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
/// 
/// ## Arguments
/// 
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `limit` - The maximum number of actors to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
pub async fn get_suggestions(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<SuggestionsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.actor.getSuggestions", host_name);

    let mut query_params = vec![];

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
            let response_body: SuggestionsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Find actor suggestions for a prefix search term. Expected use is for auto-completion during text field entry. Does not require auth.
/// 
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
/// 
/// ## Arguments
/// 
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `query` - The search term to find actors for.
/// * `limit` - The maximum number of actors to return. Defaults to 10.
pub async fn search_actors_typeahead(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    query: &str,
    limit: Option<i32>
) -> Result<app_bsky::actor::SearchActorsTypeaheadResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/app.bsky.actor.searchActorsTypeahead",
        host_name
    );

    let mut query_params = Vec::new();
    query_params.push(("q", query));

    let limit_string = limit.unwrap_or_else(|| 10).to_string();
    query_params.push(("limit", limit_string.as_str()));

    
    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: SearchActorsTypeaheadResponse =
                response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Find actors (profiles) matching search criteria. Does not require auth.
/// 
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
/// 
/// ## Arguments
/// 
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `query` - The search term to find actors for.
/// * `limit` - The maximum number of actors to return. Defaults to 25.
/// * `cursor` - A cursor for pagination.
pub async fn search_actors(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    query: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<SearchActorsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.actor.searchActors", host_name);

    let mut query_params = Vec::new();
    query_params.push(("q", query));

    let limit_string = limit.unwrap_or_else(|| 25).to_string();
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
            let response_body: SearchActorsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}
