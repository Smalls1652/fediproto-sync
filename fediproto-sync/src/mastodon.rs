use megalodon::response::Response;

pub trait MastodonApiExtensions {
    async fn get_latest_posts(
        self,
        account_id: &str,
        last_post_id: Option<String>
    ) -> Result<Response<Vec<megalodon::entities::Status>>, Box<dyn std::error::Error>>;
}

impl MastodonApiExtensions for Box<dyn megalodon::Megalodon + Send + Sync> {
    async fn get_latest_posts(
        self,
        account_id: &str,
        last_post_id: Option<String>
    ) -> Result<Response<Vec<megalodon::entities::Status>>, Box<dyn std::error::Error>> {
        let limit_value = match last_post_id {
            Some(_) => None,
            None => Some(1)
        };

        let latest_statuses_options = megalodon::megalodon::GetAccountStatusesInputOptions {
            limit: limit_value,
            max_id: None,
            since_id: last_post_id,
            pinned: Some(false),
            exclude_replies: Some(true),
            exclude_reblogs: Some(true),
            only_media: Some(false),
            only_public: Some(true)
        };

        let latest_posts = self
            .get_account_statuses(account_id.to_string(), Some(&latest_statuses_options))
            .await?;

        Ok(latest_posts)
    }
}

pub fn parse_mastodon_status_html(
    status_html: &str
) -> Result<dom_query::Document, Box<dyn std::error::Error>> {
    let parsed_html = dom_query::Document::fragment(status_html);

    Ok(parsed_html)
}

pub fn parse_mastodon_status(
    parsed_html: &dom_query::Document
) -> Result<String, Box<dyn std::error::Error>> {
    let mut stripped_html = String::new();

    for node in parsed_html.select("p").iter() {
        stripped_html.push_str(format!("{}\n\n", node.text()).as_str());
    }

    stripped_html = stripped_html.trim_end().to_string();

    Ok(stripped_html)
}

pub fn find_links_in_status(
    parsed_html: &dom_query::Document,
    tags: &Vec<megalodon::entities::status::Tag>
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut found_links = Vec::new();

    for node in parsed_html.select("a").iter() {
        let href = node.attr("href").unwrap().to_string();

        if tags.iter().any(|tag| &tag.url.to_lowercase() == &href.to_lowercase()) {
            tracing::info!("Ignoring tag link: {}", href);
            continue;
        }

        found_links.push(href);
    }

    Ok(found_links)
}

pub fn find_tags_in_status(
    parsed_html: &dom_query::Document,
    tags: &Vec<megalodon::entities::status::Tag>
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut found_tags = Vec::new();

    for node in parsed_html.select("a").iter() {
        let href = node.attr("href").unwrap().to_string();

        if tags.iter().any(|tag| &tag.url.to_lowercase() == &href.to_lowercase()) {
            found_tags.push(node.text().to_string());
        }
    }

    Ok(found_tags)
}
