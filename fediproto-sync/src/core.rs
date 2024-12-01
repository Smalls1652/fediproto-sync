use diesel::{
    Connection,
    ExpressionMethods,
    OptionalExtension,
    QueryDsl,
    RunQueryDsl,
    SqliteConnection
};

use crate::{bsky, mastodon::MastodonApiExtensions, schema, FediProtoSyncEnvVars};

pub async fn run(config: FediProtoSyncEnvVars) -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let db_connection = &mut SqliteConnection::establish(&database_url)?;
    tracing::info!("Connected to database.");

    let mut interval = tokio::time::interval(config.sync_interval);

    loop {
        interval.tick().await;

        tracing::info!("Running sync...");
        let sync_result = run_sync(config.clone(), db_connection).await;

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

async fn run_sync(config: FediProtoSyncEnvVars, db_connection: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mastodon_client = megalodon::generator(
        megalodon::SNS::Mastodon,
        format!("https://{}", config.mastodon_server.clone()),
        Some(config.mastodon_access_token.clone()),
        None
    )?;
    let account = mastodon_client.verify_account_credentials().await?;
    tracing::info!("Authenticated to Mastodon as '{}'", account.json.username);

    let bsky_auth = bsky::create_session_token(&config).await?;
    tracing::info!("Authenticated to BlueSky as '{}'", bsky_auth.session.handle);

    let last_synced_post_id = schema::mastodon_posts::table
        .order(schema::mastodon_posts::created_at.desc())
        .select(schema::mastodon_posts::post_id)
        .first::<String>(db_connection)
        .optional()?;

    let mut latest_posts = mastodon_client
        .get_latest_posts(&account.json.id, last_synced_post_id)
        .await?;
    
    latest_posts.json.reverse();

    tracing::info!(
        "Retrieved '{}' new posts from Mastodon.",
        latest_posts.json.len()
    );

    for post_item in latest_posts.json {
        tracing::info!("Processing post '{}'", post_item.id);
        bsky::sync_post(&bsky_auth, db_connection, &account.json, post_item).await?;
    }

    Ok(())
}
