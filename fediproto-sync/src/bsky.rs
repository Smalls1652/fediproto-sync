use atprotolib_rs::{
    api_calls::{ApiAuthBearerToken, ApiAuthConfig, ApiAuthConfigData},
    types::{
        app_bsky::{
            self,
            embed::{ExternalEmbed, ImageEmbed},
            feed::{PostEmbedExternal, PostEmbedImage, PostEmbeds},
            richtext::{
                ByteSlice,
                RichTextFacet,
                RichTextFacetFeature,
                RichTextFacetLink,
                RichTextFacetTag
            }
        },
        com_atproto::{self, server::CreateSessionRequest}
    }
};
use diesel::*;

use crate::{mastodon, models, schema, FediProtoSyncEnvVars};

pub struct BlueSkyAuthentication {
    pub host_name: String,
    pub auth_config: ApiAuthConfig,
    pub session: com_atproto::server::CreateSessionResponse
}

impl BlueSkyAuthentication {
    pub fn new(
        host_name: &str,
        auth_config: ApiAuthConfig,
        session: com_atproto::server::CreateSessionResponse
    ) -> Self {
        Self {
            host_name: host_name.to_string(),
            auth_config,
            session
        }
    }
}

pub async fn create_session_token(
    config: &FediProtoSyncEnvVars
) -> Result<BlueSkyAuthentication, Box<dyn std::error::Error>> {
    let initial_auth_config = ApiAuthConfig {
        data: ApiAuthConfigData::None
    };

    let bsky_create_session = atprotolib_rs::types::com_atproto::server::create_session(
        &config.bluesky_pds_server,
        &initial_auth_config,
        CreateSessionRequest {
            identifier: config.bluesky_handle.clone(),
            password: config.bluesky_app_password.clone(),
            auth_factor_token: None
        }
    )
    .await?;

    let bsky_auth_config = ApiAuthConfig {
        data: ApiAuthConfigData::BearerToken(ApiAuthBearerToken {
            token: bsky_create_session.access_jwt.clone()
        })
    };

    Ok(BlueSkyAuthentication::new(
        config.bluesky_pds_server.clone().as_str(),
        bsky_auth_config,
        bsky_create_session
    ))
}

pub async fn sync_post(
    bsky_auth: &BlueSkyAuthentication,
    db_connection: &mut diesel::SqliteConnection,
    mastodon_account: &megalodon::entities::account::Account,
    mastodon_status: &megalodon::entities::Status
) -> Result<(), Box<dyn std::error::Error>> {
    let mastodon_status = mastodon_status.clone();

    let mut post_item = generate_post_item(
        &bsky_auth.host_name,
        &bsky_auth.auth_config,
        &mastodon_status
    )
    .await?;

    let mut previous_post_id = None;

    if mastodon_status.in_reply_to_id.is_some()
        && mastodon_status
            .clone()
            .in_reply_to_account_id
            .unwrap_or_else(|| "".to_string())
            == mastodon_account.id.clone()
    {
        let in_reply_to_id = mastodon_status.in_reply_to_id.clone().unwrap();

        let previous_mastodon_post = resolve_previous_post(db_connection, &in_reply_to_id).await?;

        let previous_synced_post = get_synced_post(db_connection, &in_reply_to_id).await?;

        let previous_synced_post_root = match previous_mastodon_post.root_mastodon_post_id {
            Some(root_mastodon_post_id) => {
                previous_post_id = Some(root_mastodon_post_id.clone());

                get_synced_post(db_connection, &root_mastodon_post_id).await?
            }
            None => {
                previous_post_id = Some(in_reply_to_id.clone());

                previous_synced_post.clone()
            }
        };

        post_item.reply_ref = Some(app_bsky::feed::PostReplyRef {
            root: com_atproto::repo::StrongRef {
                cid: previous_synced_post_root.bsky_post_cid.clone(),
                uri: previous_synced_post_root.bsky_post_uri.clone()
            },
            parent: com_atproto::repo::StrongRef {
                cid: previous_synced_post.bsky_post_cid.clone(),
                uri: previous_synced_post.bsky_post_uri.clone()
            }
        });
    }

    let apply_write_request = com_atproto::repo::ApplyWritesRequest {
        repo: bsky_auth.session.did.clone(),
        validate: true,
        writes: vec![com_atproto::repo::ApplyWritesRequestWrites::Create(
            com_atproto::repo::Create::new(
                "app.bsky.feed.post",
                com_atproto::repo::ApplyWritesValue::Post(post_item)
            )
        )],
        swap_commit: None
    };

    // let apply_write_request_json =
    // serde_json::to_string_pretty(&apply_write_request)?;

    let apply_write_result = com_atproto::repo::apply_writes(
        &bsky_auth.host_name,
        &bsky_auth.auth_config,
        apply_write_request
    )
    .await;

    match apply_write_result {
        Ok(result) => {
            let post_result = match result.results.first().unwrap() {
                com_atproto::repo::ApplyWritesResponseResults::CreateResult(create_result) => {
                    create_result
                }

                _ => panic!("Unexpected response from Bluesky")
            };

            diesel::insert_into(schema::mastodon_posts::table)
                .values(models::NewMastodonPost::new(
                    &mastodon_status,
                    Some(post_result.cid.clone()),
                    previous_post_id
                ))
                .execute(db_connection)?;

            diesel::insert_into(schema::synced_posts::table)
                .values(models::NewSyncedPost::new(
                    &mastodon_status.id,
                    &post_result.cid,
                    &post_result.uri
                ))
                .execute(db_connection)?;

            tracing::info!("Synced post '{}' to BlueSky.", mastodon_status.id);

            Ok(())
        }

        Err(error) => {
            tracing::error!("Error syncing post '{}': {}", mastodon_status.id, error);
            Err(error)
        }
    }
}

pub async fn generate_post_item(
    host_name: &str,
    auth_config: &ApiAuthConfig,
    mastodon_status: &megalodon::entities::Status
) -> Result<app_bsky::feed::Post, Box<dyn std::error::Error>> {
    let (mastodon_status_send, mastodon_status_recv) = std::sync::mpsc::channel();
    let (html_parse_tx, html_parse_rx) = std::sync::mpsc::channel();

    mastodon_status_send.send(mastodon_status.clone()).unwrap();
    std::thread::spawn(move || {
        let recieved_status = mastodon_status_recv.recv().unwrap();

        let parsed_status = mastodon::ParsedMastodonPost::from_mastodon_status(&recieved_status).unwrap();

        html_parse_tx.send(parsed_status).unwrap();
    });

    let mut parsed_status = html_parse_rx.recv().unwrap();
    parsed_status.truncate_post_content()?;

    let mut post_item = app_bsky::feed::Post::new(&parsed_status.stripped_html, parsed_status.mastodon_status.created_at, None);

    let mut richtext_facets = Vec::new();

    if parsed_status.mastodon_status.media_attachments.len() > 0 {
        tracing::info!(
            "Found '{}' media attachments in post '{}'",
            parsed_status.mastodon_status.media_attachments.len(),
            parsed_status.mastodon_status.id
        );
        let mut image_attachments = Vec::new();

        let media_attachment_client = reqwest::Client::new();
        for media_attachment in parsed_status.mastodon_status.media_attachments {
            if media_attachment.r#type != megalodon::entities::attachment::AttachmentType::Image {
                tracing::warn!(
                    "Skipping non-image media attachment '{}'",
                    media_attachment.url
                );
                continue;
            }

            let media_attachment_response = media_attachment_client
                .get(&media_attachment.url)
                .send()
                .await?;

            let media_attachment_bytes = media_attachment_response.bytes().await?;

            let blob_upload_response = com_atproto::repo::upload_blob(
                host_name,
                auth_config,
                media_attachment_bytes.to_vec(),
                Some("image/jpeg")
            )
            .await?;

            image_attachments.push(ImageEmbed {
                image: blob_upload_response.blob,
                alt: media_attachment
                    .description
                    .unwrap_or_else(|| "".to_string()),
                aspect_ratio: None
            });

            tracing::info!(
                "Uploaded media attachment '{}' to Bluesky",
                media_attachment.url
            );
        }

        post_item.embed = Some(PostEmbeds::Images(PostEmbedImage {
            images: image_attachments
        }));
    }

    if parsed_status.found_tags.len() > 0 {
        for tag in parsed_status.found_tags {
            let tag_start_index = parsed_status.stripped_html.find(tag.as_str()).unwrap();
            let tag_end_index = tag_start_index + tag.len();

            let richtext_facet_tag = RichTextFacet {
                index: ByteSlice {
                    byte_start: tag_start_index as i64,
                    byte_end: tag_end_index as i64
                },
                features: vec![RichTextFacetFeature::Tag(RichTextFacetTag {
                    tag: tag.trim_start_matches("#").to_string()
                })]
            };

            richtext_facets.push(richtext_facet_tag);
        }
    }

    if parsed_status.found_links.len() > 0 {
        tracing::info!(
            "Found '{}' links in post '{}'",
            parsed_status.found_links.len(),
            mastodon_status.id
        );
        let first_link = parsed_status.found_links[0].clone();

        let link_metadata = get_link_metadata(&first_link).await?;

        if post_item.embed.is_none() {
            tracing::info!(
                "Post has no embeds, adding external embed for link '{}'",
                first_link
            );
            let link_thumbnail_url = link_metadata["image"].as_str().unwrap_or_else(|| "");
            let link_thumbnail_bytes = match link_thumbnail_url == "" {
                    true => vec![],
                    false => get_link_thumbnail(link_thumbnail_url).await?
            };


            let blob_item = match link_thumbnail_bytes.len() > 0 {
                true => Some(com_atproto::repo::upload_blob(
                    host_name,
                    auth_config,
                    link_thumbnail_bytes,
                    Some("image/jpeg")
                ).await?.blob),

                _ => None
            };

            post_item.embed = Some(PostEmbeds::External(PostEmbedExternal {
                external: ExternalEmbed {
                    uri: link_metadata["url"].as_str().unwrap().to_string(),
                    title: link_metadata["title"].as_str().unwrap().to_string(),
                    description: link_metadata["description"].as_str().unwrap().to_string(),
                    thumb: blob_item
                }
            }));
        }

        let link_start_index = parsed_status.stripped_html.find(first_link.as_str()).unwrap();
        let link_end_index = link_start_index + first_link.len();

        let richtext_facet_link = RichTextFacet {
            index: ByteSlice {
                byte_start: link_start_index as i64,
                byte_end: link_end_index as i64
            },
            features: vec![RichTextFacetFeature::Link(RichTextFacetLink {
                uri: first_link
            })]
        };

        richtext_facets.push(richtext_facet_link);
    }

    post_item.facets = match richtext_facets.len() > 0 {
        true => Some(richtext_facets),
        false => None
    };

    Ok(post_item)
}

pub async fn resolve_previous_post(
    db_connection: &mut diesel::SqliteConnection,
    previous_post_id: &str
) -> Result<models::MastodonPost, Box<dyn std::error::Error>> {
    let previous_post = schema::mastodon_posts::table
        .filter(schema::mastodon_posts::post_id.eq(previous_post_id))
        .first::<models::MastodonPost>(db_connection)?;

    Ok(previous_post)
}

pub async fn get_synced_post(
    db_connection: &mut diesel::SqliteConnection,
    mastodon_post_id: &str
) -> Result<models::SyncedPost, Box<dyn std::error::Error>> {
    let synced_post = schema::synced_posts::table
        .filter(schema::synced_posts::mastodon_post_id.eq(mastodon_post_id))
        .first::<models::SyncedPost>(db_connection)?;

    Ok(synced_post)
}

pub async fn get_link_metadata(url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    tracing::info!("Getting link metadata for '{}'.", url);
    let client = reqwest::Client::new();

    let query_params = vec![("url", url)];

    let link_info_response = client
        .get("https://cardyb.bsky.app/v1/extract")
        .query(&query_params)
        .send()
        .await?;

    let link_info_json = link_info_response.json::<serde_json::Value>().await?;

    Ok(link_info_json)
}

pub async fn get_link_thumbnail(image_url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    tracing::info!("Getting link thumbnail for '{}'.", image_url);
    let client = reqwest::Client::new();

    let link_thumbnail_response = client.get(image_url).send().await?;

    let link_thumbnail_bytes = link_thumbnail_response.bytes().await?;

    Ok(link_thumbnail_bytes.to_vec())
}
