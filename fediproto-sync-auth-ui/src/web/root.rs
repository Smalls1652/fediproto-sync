use anyhow::Context;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use super::check_for_existing_token;
use crate::{FediProtoSyncWebServerAppState, error::FediProtoSyncWebError};

pub async fn root_endpoint(
    State(app_state): State<FediProtoSyncWebServerAppState>
) -> Result<impl IntoResponse, FediProtoSyncWebError> {
    let db_connection = &mut app_state
        .db_pool
        .get()
        .context("Failed to get the database connection.")?;

    let mastodon_token_exists = check_for_existing_token(db_connection, "mastodon")?;

    let mut html_output =
        "<html>\n<head>\n<title>FediProtoSync</title>\n</head>\n<body>\n".to_string();

    match mastodon_token_exists {
        true => html_output.push_str(
            "<h1>Mastodon</h1>\n<p><font style=\"font-style: bold;\">Configured</font></p>\n",
        ),
        false => html_output
            .push_str("<h1>Mastodon</h1>\n<p><a href=\"/auth/mastodon/login\">Configure</a></p>\n"),
    }

    html_output.push_str("</body>\n</html>");

    Ok(((), Html(html_output)))
}
