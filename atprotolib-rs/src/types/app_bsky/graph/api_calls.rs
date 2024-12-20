use super::{
    api_requests::{
        MuteActorListRequest,
        MuteActorRequest,
        MuteThreadRequest,
        UnmuteActorListRequest,
        UnmuteActorRequest,
        UnmuteThreadRequest
    },
    api_responses::{
        ActorStarterPacksResponse,
        BlocksResponse,
        FollowersResponse,
        FollowsResponse,
        KnownFollowersResponse,
        ListBlocksResponse,
        ListMutesResponse,
        ListResponse,
        ListsResponse,
        MutesResponse,
        StarterPackResponse,
        SuggestedFollowsByActorResponse
    }
};
use crate::api_calls::{AddApiAuth, ApiAuthConfig, ApiError};

/// Get a list of starter packs created by the actor.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `actor` - Handle or DID of the account to fetch the starter packs of.
/// * `limit` - The maximum number of starter packs to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
pub async fn get_actor_starter_packs(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<ActorStarterPacksResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/app.bsky.graph.getActorStarterPacks",
        host_name
    );

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
            let response_body: ActorStarterPacksResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Enumerates which accounts the requesting account is currently blocking.
/// Requires auth.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `limit` - The maximum number of accounts to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
pub async fn get_blocks(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<BlocksResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getBlocks", host_name);

    let mut query_params = Vec::new();

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
            let response_body: BlocksResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Enumerates accounts which follow a specified account (actor).
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `actor` - Handle or DID of the account to fetch the followers of.
/// * `limit` - The maximum number of followers to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
pub async fn get_followers(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<FollowersResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getFollowers", host_name);

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
            let response_body: FollowersResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Enumerates accounts which a specified account (actor) follows.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `actor` - Handle or DID of the account to fetch the profiles of.
/// * `limit` - The maximum number of profiles to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
pub async fn get_follows(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<FollowsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getFollows", host_name);

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
            let response_body: FollowsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Enumerates accounts which follow a specified account (actor) and are
/// followed by the viewer.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `actor` - Handle or DID of the account to fetch the followers of.
/// * `limit` - The maximum number of followers to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
pub async fn get_known_followers(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<KnownFollowersResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/app.bsky.graph.getKnownFollowers",
        host_name
    );

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
            let response_body: KnownFollowersResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Get mod lists that the requesting account (actor) is blocking. Requires
/// auth.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `limit` - The maximum number of mod lists to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
pub async fn get_list_blocks(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<ListBlocksResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getListBlocks", host_name);

    let mut query_params = Vec::new();

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
            let response_body: ListBlocksResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Enumerates mod lists that the requesting account (actor) currently has
/// muted. Requires auth.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `limit` - The maximum number of mod lists to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
pub async fn get_list_mutes(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<ListMutesResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getListMutes", host_name);

    let mut query_params = Vec::new();

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
            let response_body: ListMutesResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Gets a 'view' (with additional context) of a specified list.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `list` - The list to fetch.
/// * `limit` - The maximum number of items to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
pub async fn get_list(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    list: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<ListResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getList", host_name);

    let mut query_params = Vec::new();
    query_params.push(("list", list));

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
            let response_body: ListResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Enumerates the lists created by a specified account (actor).
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `actor` - Handle or DID of the account to fetch the lists of.
/// * `limit` - The maximum number of lists to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
pub async fn get_lists(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    actor: &str,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<ListsResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getLists", host_name);

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
            let response_body: ListsResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Enumerates accounts that the requesting account (actor) currently has muted.
/// Requires auth.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `limit` - The maximum number of accounts to return. Defaults to 50.
/// * `cursor` - A cursor for pagination.
pub async fn get_mutes(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    limit: Option<i32>,
    cursor: Option<&str>
) -> Result<MutesResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getMutes", host_name);

    let mut query_params = Vec::new();

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
            let response_body: MutesResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Gets a view of a starter pack.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `starter_pack` - The URI of the starter pack to fetch.
pub async fn get_starter_pack(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    starter_pack: &str
) -> Result<StarterPackResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.getStarterPack", host_name);

    let query_params = vec![("starter_pack", starter_pack)];

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: StarterPackResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Enumerates follows similar to a given account (actor). Expected use is to
/// recommend additional accounts immediately after following one account.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `actor` - Handle or DID of the account to fetch the suggested follows for.
pub async fn get_suggested_follows_by_actor(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    actor: &str
) -> Result<SuggestedFollowsByActorResponse, Box<dyn std::error::Error>> {
    let api_url = format!(
        "https://{}/xrpc/app.bsky.graph.getSuggestedFollowsByActor",
        host_name
    );

    let query_params = vec![("actor", actor)];

    let response = client
        .get(&api_url)
        .query(&query_params)
        .add_api_auth(api_auth_config.clone())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response_body: SuggestedFollowsByActorResponse = response.json().await?;
            Ok(response_body)
        }
        _ => Err(Box::new(ApiError::new(response).await?))
    }
}

/// Creates a mute relationship for the specified list of accounts. Mutes are
/// private in Bluesky. Requires auth.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to mute a list of actors.
pub async fn mute_actor_list(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: MuteActorListRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.muteActorList", host_name);

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

/// Creates a mute relationship for the specified account. Mutes are private in
/// Bluesky. Requires auth.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to mute an actor.
pub async fn mute_actor(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: MuteActorRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.muteActor", host_name);

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

/// Mutes a thread preventing notifications from the thread and any of its
/// children. Mutes are private in Bluesky. Requires auth.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to mute a thread.
pub async fn mute_thread(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: MuteThreadRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.muteThread", host_name);

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

/// Unmutes the specified list of accounts. Requires auth.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to unmute a list of actors.
pub async fn unmute_actor_list(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: UnmuteActorListRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.unmuteActorList", host_name);

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

/// Unmutes the specified account. Requires auth.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to unmute an actor.
pub async fn unmute_actor(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: UnmuteActorRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.unmuteActor", host_name);

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

/// Unmutes the specified thread. Requires auth.
///
/// <div class="warning">Requires the <code>apicalls</code> feature.</div>
///
/// ## Arguments
///
/// * `host_name` - The host name of the server to make the request to.
/// * `api_auth_config` - The API authentication configuration.
/// * `request` - The request to unmute a thread.
pub async fn unmute_thread(
    host_name: &str,
    client: reqwest::Client,
    api_auth_config: &ApiAuthConfig,
    request: UnmuteThreadRequest
) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("https://{}/xrpc/app.bsky.graph.unmuteThread", host_name);

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
