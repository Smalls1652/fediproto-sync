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

#[derive(Debug, Clone)]
pub struct ParsedMastodonPost {
    pub mastodon_status: megalodon::entities::Status,
    pub stripped_html: String,
    pub found_links: Vec<String>,
    pub found_tags: Vec<String>
}

impl ParsedMastodonPost {
    pub fn from_mastodon_status(
        status: &megalodon::entities::Status
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let html_document = dom_query::Document::fragment(status.content.clone().as_str());

        let mastodon_status = status.clone();
        let stripped_html = Self::convert_html_content_to_string(&html_document)?;
        let found_links = Self::get_links(&html_document, &status.tags)?;
        let found_tags = Self::get_tags(&html_document, &status.tags)?;

        Ok(Self {
            mastodon_status,
            stripped_html,
            found_links,
            found_tags
        })
    }

    pub fn truncate_post_content(
        &mut self
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.stripped_html.len() <= 300 {
            return Ok(());
        }

        let ellipsis_string = "[...]";
        let read_more_string = format!("\n\nRead more: {}", self.mastodon_status.url.as_ref().unwrap());

        let cut_down_length = 300 - ellipsis_string.len() - read_more_string.len();

        let mut string_builder = Self::trim_post_string(&self.stripped_html, cut_down_length);

        let mut trimmed_tags = Vec::new();

        for tag in &self.found_tags {
            if !string_builder.contains(&*tag) {
                trimmed_tags.push(tag.clone());
            }
        }

        let mut trimmed_tag_string_builder = String::new();
        trimmed_tag_string_builder.push_str("\n\n");
        for (index, tag) in trimmed_tags.iter().enumerate() {
            trimmed_tag_string_builder.push_str(tag.as_str());

            if index < trimmed_tags.len() {
                trimmed_tag_string_builder.push_str(" ");
            }
        }

        let final_cut_down_length = cut_down_length - trimmed_tag_string_builder.len();

        string_builder = Self::trim_post_string(&self.stripped_html, final_cut_down_length);
        
        string_builder.push_str(ellipsis_string);
        string_builder.push_str(read_more_string.as_str());
        string_builder.push_str(trimmed_tag_string_builder.as_str());

        self.stripped_html = string_builder;
        self.found_links = vec![
            self.mastodon_status.url.as_ref().unwrap().to_string()
        ];

        Ok(())
    }

    fn trim_post_string(
        content: &str,
        max_length: usize
    ) -> String {
        if content.len() <= max_length {
            return content.to_string();
        }

        let mut string_builder = String::new();

        for (index, character) in content.chars().enumerate() {
            if index >= max_length - 1 {
                break;
            }

            string_builder.push(character);
        }

        string_builder
    }

    fn convert_html_content_to_string(
        document: &dom_query::Document
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut stripped_html = String::new();

        for node in document.select("p").iter() {
            stripped_html.push_str(format!("{}\n\n", node.text()).as_str());
        }

        stripped_html = stripped_html.trim_end().to_string();

        Ok(stripped_html)
    }

    fn get_links(
        document: &dom_query::Document,
        tags: &Vec<megalodon::entities::status::Tag>
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut links = Vec::new();

        for node in document.select("a").iter() {
            let href = node.attr("href").unwrap().to_string();

            if tags
                .iter()
                .any(|tag| &tag.url.to_lowercase() == &href.to_lowercase())
            {
                tracing::info!("Ignoring tag link: {}", href);
                continue;
            }

            links.push(href);
        }

        Ok(links)
    }

    fn get_tags(
        document: &dom_query::Document,
        tags: &Vec<megalodon::entities::status::Tag>
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut found_tags = Vec::new();

        for node in document.select("a").iter() {
            let href = node.attr("href").unwrap().to_string();

            if tags
                .iter()
                .any(|tag| &tag.url.to_lowercase() == &href.to_lowercase())
            {
                found_tags.push(node.text().to_string());
            }
        }

        Ok(found_tags)
    }
}
