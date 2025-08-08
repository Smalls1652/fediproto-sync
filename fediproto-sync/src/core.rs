use std::sync::Arc;

use anyhow::Result;
use atrium_api::{
    agent::{atp_agent::AtpAgent, atp_agent::store::MemorySessionStore},
    client::AtpServiceClient,
    types::string::Did,
};
use atrium_xrpc_client::reqwest::{ReqwestClient, ReqwestClientBuilder};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use fediproto_sync_db::{
    AnyConnection,
    models::{self, CachedServiceTokenDecrypt, NewMastodonPostRetryQueueItem},
};
use fediproto_sync_lib::{
    config::FediProtoSyncConfig,
    error::{AuthenticationSource, FediProtoSyncError},
};
use megalodon::{Megalodon, entities::Account};

use crate::{bsky_post_sync, mastodon::MastodonApiExtensions};

pub struct SocialMediaClients {
    /// The ATProto agent for the FediProto Sync application.
    pub atp_agent: AtpAgent<MemorySessionStore, ReqwestClient>,

    pub atp_pds_service_endpoint: String,

    pub atp_did: Did,

    /// The Mastodon client for the FediProto Sync application.
    pub mastodon_client: Box<dyn Megalodon + Send + Sync>,
}

impl SocialMediaClients {
    async fn new(
        config: &FediProtoSyncConfig,
        db_connection_pool: &Pool<ConnectionManager<AnyConnection>>,
    ) -> Result<SocialMediaClients> {
        let atp_auth_data = create_atp_agent(config).await?;
        let atp_agent = atp_auth_data.0;
        let atp_pds_service_endpoint = atp_auth_data.1.replace("https://", "");
        let atp_did = atp_auth_data.2;

        let db_connection = &mut db_connection_pool.get()?;
        let mastodon_client = create_mastodon_client(config, db_connection).await?;

        Ok(SocialMediaClients {
            atp_agent,
            atp_pds_service_endpoint,
            atp_did,
            mastodon_client,
        })
    }
}

pub async fn run_sync_loop(
    config: &FediProtoSyncConfig,
    db_connection_pool: Pool<ConnectionManager<AnyConnection>>,
) -> Result<()> {
    let social_media_clients = SocialMediaClients::new(config, &db_connection_pool).await?;
    let social_media_clients = Arc::new(social_media_clients);

    let mastodon_account = social_media_clients
        .mastodon_client
        .verify_account_credentials()
        .await
        .map_err(|_| FediProtoSyncError::AuthenticationError(AuthenticationSource::Mastodon))?
        .json;

    let mut interval = tokio::time::interval(config.sync_interval);
    loop {
        interval.tick().await;

        tracing::info!("Running sync...");

        let sync_result = sync_posts(
            config,
            &db_connection_pool,
            social_media_clients.clone(),
            &mastodon_account,
        )
        .await;

        match sync_result {
            Ok(_) => {
                tracing::info!("Sync completed successfully.");
            }
            Err(e) => {
                tracing::error!("Sync failed: {:#?}", e);
            }
        }
    }
}

async fn sync_posts(
    config: &FediProtoSyncConfig,
    db_connection_pool: &Pool<ConnectionManager<AnyConnection>>,
    social_media_clients: Arc<SocialMediaClients>,
    mastodon_account: &Account,
) -> Result<()> {
    let db_connection = &mut db_connection_pool.get()?;

    // Get the last synced post ID, if any.
    tracing::info!("Getting last synced post...");
    let last_synced_post_id =
        fediproto_sync_db::operations::get_last_synced_mastodon_post_id(db_connection)?;

    // Get the latest posts from Mastodon.
    // If there is no last synced post ID, we will only get the latest post.
    // Otherwise, we will get all posts since the last synced post.
    tracing::info!("Getting latest posts from Mastodon...");
    let mut latest_posts = social_media_clients
        .mastodon_client
        .get_latest_posts(
            &mastodon_account.id,
            last_synced_post_id.clone(),
            config.mastodon_allow_unlisted_posts,
        )
        .await?;

    // Reverse the posts so we process them in ascending order.
    latest_posts.reverse();

    // If there is no last synced post ID, we need to add the initial post to the
    // database. This is so we have a starting point for future syncs.
    //
    // Note: The initial post **is not synced** to BlueSky.
    if last_synced_post_id.is_none() && latest_posts.len() > 0 {
        let initial_post = latest_posts[0].clone();

        let new_mastodon_post = models::NewMastodonPost::new(&initial_post, None, None);
        fediproto_sync_db::operations::insert_new_synced_mastodon_post(
            db_connection,
            &new_mastodon_post,
        )?;

        tracing::info!("Added initial post to database for future syncs.");

        return Ok(());
    }

    tracing::info!(
        "Retrieved '{}' new posts from Mastodon.",
        latest_posts.len()
    );

    let posts_to_retry =
        fediproto_sync_db::operations::get_mastodon_post_retry_queue_items(db_connection)?;

    // Filter out any posts that are in the retry queue so we don't try to process
    // them twice.
    let latest_posts = latest_posts
        .iter()
        .filter(|post| {
            let post_id = post.id.clone();
            let post_id = post_id.parse::<i64>().unwrap_or(0);

            let retry_post_exists = posts_to_retry.iter().any(|retry_item| {
                let retry_post_id = retry_item.id;
                retry_post_id == post_id
            });

            !retry_post_exists
        })
        .cloned()
        .collect::<Vec<megalodon::entities::Status>>();

    if posts_to_retry.len() > 0 {
        tracing::info!(
            "Retrying '{}' posts that failed to sync previously.",
            posts_to_retry.len()
        );

        for retry_item in posts_to_retry {
            let fetched_post = social_media_clients
                .mastodon_client
                .get_status(retry_item.id.to_string())
                .await;

            match fetched_post {
                Ok(post) => {
                    tracing::info!("Retrying sync for post '{}'", retry_item.id);
                    let post = &post.json;

                    let sync_config = bsky_post_sync::BlueSkyPostSyncConfig {
                        config: config.clone(),
                        did: social_media_clients.atp_did.clone(),
                        pds_service_endpoint: social_media_clients.atp_pds_service_endpoint.clone(),
                        mastodon_account: mastodon_account.clone(),
                        db_connection_pool: db_connection_pool.clone(),
                    };

                    let sync_result = bsky_post_sync::sync_post(
                        &post,
                        &social_media_clients.atp_agent,
                        &sync_config,
                    )
                    .await;

                    match sync_result {
                        Ok(_) => {
                            tracing::info!("Post '{}' processed successfully.", retry_item.id);
                            fediproto_sync_db::operations::delete_mastodon_post_retry_queue_item(
                                db_connection,
                                &retry_item,
                            )?;
                        }

                        Err(e) => {
                            tracing::error!("Failed to process post '{}': {:#?}", retry_item.id, e);

                            let source_error = e.source();

                            if let Some(source_error) = source_error {
                                tracing::error!("Source error: {:#?}", source_error);
                            }

                            fediproto_sync_db::operations::update_mastodon_post_retry_queue_item(
                                db_connection,
                                &retry_item,
                                None,
                            )?;
                        }
                    }
                }

                Err(e) => {
                    tracing::warn!("Failed to fetch post '{}': {:#?}", retry_item.id, e);
                    tracing::warn!("Removing post from retry queue.");

                    fediproto_sync_db::operations::delete_mastodon_post_retry_queue_item(
                        db_connection,
                        &retry_item,
                    )?;

                    continue;
                }
            }
        }
    }

    // Process each new post and sync it to BlueSky.
    for post_item in latest_posts {
        tracing::info!("Processing post '{}'", post_item.id);

        let sync_config = bsky_post_sync::BlueSkyPostSyncConfig {
            config: config.clone(),
            did: social_media_clients.atp_did.clone(),
            pds_service_endpoint: social_media_clients.atp_pds_service_endpoint.clone(),
            mastodon_account: mastodon_account.clone(),
            db_connection_pool: db_connection_pool.clone(),
        };

        let sync_result =
            bsky_post_sync::sync_post(&post_item, &social_media_clients.atp_agent, &sync_config)
                .await;

        match sync_result {
            Ok(_) => {
                tracing::info!("Post '{}' processed successfully.", post_item.id);
            }
            Err(error) => {
                tracing::error!("Failed to process post '{}': {:#?}", post_item.id, error);

                let source_error = error.source();

                if let Some(source_error) = source_error {
                    tracing::error!("Source error: {:#?}", source_error);
                }

                let new_retry_item = NewMastodonPostRetryQueueItem::new(
                    &post_item.id.parse::<i64>()?,
                    error.to_string().as_str(),
                );

                fediproto_sync_db::operations::insert_mastodon_post_retry_queue_item(
                    db_connection,
                    &new_retry_item,
                )?;
            }
        }
    }

    let cached_files_to_delete =
        fediproto_sync_db::operations::get_cached_file_records(db_connection)?;

    if cached_files_to_delete.len() > 0 {
        tracing::info!("Deleting cached files during sync...");

        for cached_file in cached_files_to_delete {
            tracing::info!("Deleting cached file '{}'.", cached_file.file_path);
            cached_file.remove_file(db_connection).await?;
        }
    }

    Ok(())
}

pub async fn create_atp_agent(
    config: &FediProtoSyncConfig
) -> Result<(AtpAgent<MemorySessionStore, ReqwestClient>, String, Did), FediProtoSyncError> {
    let client = ReqwestClientBuilder::new(format!("https://{}", &config.bluesky_pds_server))
        .client(create_http_client(config)?)
        .build();

    let atp_agent = AtpAgent::new(client, MemorySessionStore::default());

    let auth_result = atp_agent
        .login(&config.bluesky_handle, &config.bluesky_app_password)
        .await
        .map_err(|_| FediProtoSyncError::AuthenticationError(AuthenticationSource::BlueSky))?;

    tracing::info!(
        "Authenticated to BlueSky as '{}'",
        auth_result.handle.as_str()
    );

    let pds_endpoint = atp_agent.get_endpoint().await;

    Ok((atp_agent, pds_endpoint, auth_result.did.clone()))
}

#[allow(dead_code)]
pub fn create_atp_service_client(
    hostname: &str,
    auth_token: Option<&str>,
    config: &FediProtoSyncConfig,
) -> Result<AtpServiceClient<ReqwestClient>, FediProtoSyncError> {
    let mut headers = reqwest::header::HeaderMap::new();
    if let Some(auth_token) = auth_token {
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(format!("Bearer {}", auth_token).as_str())
                .map_err(|_| FediProtoSyncError::HttpClientCreationError)?,
        );
    }

    let client = ReqwestClientBuilder::new(format!("https://{}", hostname))
        .client(
            reqwest::Client::builder()
                .user_agent(config.user_agent.clone())
                .use_rustls_tls()
                .default_headers(headers)
                .build()
                .map_err(|_| FediProtoSyncError::HttpClientCreationError)?,
        )
        .build();

    let service_client = AtpServiceClient::new(client);

    Ok(service_client)
}

/// Create a Mastodon client with `Megalodon`.
///
/// ## Arguments
///
/// * `config` - The environment variables for the FediProto Sync application.
/// * `db_connection_pool` - The database connection pool for the FediProto Sync application.
async fn create_mastodon_client(
    config: &FediProtoSyncConfig,
    db_connection: &mut PooledConnection<ConnectionManager<AnyConnection>>,
) -> Result<Box<dyn Megalodon + Send + Sync>, FediProtoSyncError> {
    let cached_mastodon_token =
        fediproto_sync_db::operations::get_cached_service_token_by_service_name(
            db_connection,
            "mastodon",
        )
        .map_err(|_| FediProtoSyncError::AuthenticationError(AuthenticationSource::Mastodon))?;

    let cached_mastodon_token = match cached_mastodon_token {
        Some(token) => token,
        None => {
            return Err(
                FediProtoSyncError::AuthenticationError(AuthenticationSource::Mastodon).into(),
            );
        }
    };

    let decrypted_mastodon_token =
        cached_mastodon_token.decrypt_access_token(&config.token_encryption_private_key)?;

    // Create the Mastodon client and authenticate.
    let mastodon_client = megalodon::generator(
        megalodon::SNS::Mastodon,
        format!("https://{}", config.mastodon_server.clone()),
        Some(decrypted_mastodon_token),
        Some(config.user_agent.clone()),
    )
    .map_err(|_| FediProtoSyncError::AuthenticationError(AuthenticationSource::Mastodon))?;

    Ok(mastodon_client)
}

/// Create a new HTTP client for the FediProto Sync application.
///
/// ## Arguments
///
/// * `config` - The environment variables for the FediProto Sync application.
pub fn create_http_client(
    config: &FediProtoSyncConfig
) -> Result<reqwest::Client, FediProtoSyncError> {
    reqwest::Client::builder()
        .user_agent(config.user_agent.clone())
        .use_rustls_tls()
        .build()
        .map_err(|_| FediProtoSyncError::HttpClientCreationError)
}
