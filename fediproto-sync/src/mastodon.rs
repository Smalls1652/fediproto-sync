use megalodon::response::Response;

/// Extension trait for the Mastodon API.
pub trait MastodonApiExtensions {
    async fn get_latest_posts(
        &self,
        account_id: &str,
        last_post_id: Option<String>
    ) -> Result<Vec<megalodon::entities::Status>, Box<dyn std::error::Error>>;
}

impl MastodonApiExtensions for Box<dyn megalodon::Megalodon + Send + Sync> {
    /// Get the latest posts from a Mastodon account.
    ///
    /// ## Arguments
    ///
    /// * `account_id` - The Mastodon account ID to get the latest posts for.
    /// * `last_post_id` - The last post ID to get posts since.
    async fn get_latest_posts(
        &self,
        account_id: &str,
        last_post_id: Option<String>
    ) -> Result<Vec<megalodon::entities::Status>, Box<dyn std::error::Error>> {
        let limit_value = match last_post_id {
            Some(_) => None,
            None => Some(1)
        };

        let latest_statuses_options = megalodon::megalodon::GetAccountStatusesInputOptions {
            limit: limit_value,
            max_id: None,
            since_id: last_post_id,
            pinned: Some(true),
            exclude_replies: Some(true),
            exclude_reblogs: Some(true),
            only_media: Some(false),
            only_public: Some(true)
        };

        let latest_posts = self
            .get_account_statuses(account_id.to_string(), Some(&latest_statuses_options))
            .await?;

        let mut filtered_latest_posts = Vec::new();

        for post in latest_posts.json {
            match post.visibility {
                megalodon::entities::status::StatusVisibility::Public => filtered_latest_posts.push(post),

                megalodon::entities::StatusVisibility::Unlisted => filtered_latest_posts.push(post),

                _ => continue
            }
        }

        Ok(filtered_latest_posts)
    }
}

/// Holds data for a parsed Mastodon post.
#[derive(Debug, Clone)]
pub struct ParsedMastodonPost {
    pub mastodon_status: megalodon::entities::Status,
    pub stripped_html: String,
    pub found_links: Vec<String>,
    pub found_tags: Vec<String>
}

impl ParsedMastodonPost {
    /// Create a new instance of the `ParsedMastodonPost` struct from a Mastodon
    /// status.
    ///
    /// ## Arguments
    ///
    /// * `status` - The Mastodon status to parse.
    pub fn from_mastodon_status(
        status: &megalodon::entities::Status
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Parse the HTML content of the status.
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

    /// Truncate the post content to ensure it fits within the 300 character
    /// limit for BlueSky.
    ///
    /// ## Note
    ///
    /// If the current content is already less than or equal to 300 characters,
    /// this method will just return without modifying the content.
    pub fn truncate_post_content(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // If the content is already less than or equal to 300 characters, we don't need
        // to truncate.
        if self.stripped_html.len() <= 300 {
            return Ok(());
        }

        // Define the ellipsis and read more, with the URL to the post on Mastodon,
        // strings.
        let ellipsis_string = "[...]";
        let read_more_string = format!(
            "\n\nRead more: {}",
            self.mastodon_status.url.as_ref().unwrap()
        );

        // Calculate the length of the truncated content after the ellipsis and read
        // more strings are added.
        let cut_down_length = 300 - ellipsis_string.len() - read_more_string.len();

        // Truncate the content.
        let mut string_builder = Self::trim_post_string(&self.stripped_html, cut_down_length);

        // Determine which tags/hashtags were removed from the content
        // after truncation.
        let mut trimmed_tags = Vec::new();

        for tag in &self.found_tags {
            if !string_builder.contains(&*tag) {
                trimmed_tags.push(tag.clone());
            }
        }

        // Build the trimmed tag string.
        let mut trimmed_tag_string_builder = String::new();
        trimmed_tag_string_builder.push_str("\n\n");
        for (index, tag) in trimmed_tags.iter().enumerate() {
            trimmed_tag_string_builder.push_str(tag.as_str());

            if index < trimmed_tags.len() {
                trimmed_tag_string_builder.push_str(" ");
            }
        }

        // Calculate the final length of the content after the tags are added.
        let final_cut_down_length = cut_down_length - trimmed_tag_string_builder.len();

        // Truncate the content again to ensure the tags fit within the 300 character
        // limit.
        string_builder = Self::trim_post_string(&self.stripped_html, final_cut_down_length);

        // Add the ellipsis, read more, and tags to the content.
        string_builder.push_str(ellipsis_string);
        string_builder.push_str(read_more_string.as_str());
        string_builder.push_str(trimmed_tag_string_builder.as_str());

        // Update the content with the truncated content and replace the found links
        // with the link to the Mastodon post.
        self.stripped_html = string_builder;
        self.found_links = vec![self.mastodon_status.url.as_ref().unwrap().to_string()];

        Ok(())
    }

    /// Trim the post content to the specified length.
    ///
    /// ## Arguments
    ///
    /// * `content` - The content to trim.
    /// * `max_length` - The maximum length of the content.
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

    /// Convert the HTML content of a Mastodon post to a string.
    ///
    /// ## Arguments
    ///
    /// * `document` - The HTML document to convert to a string.
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

    /// Get the links from the HTML content of a Mastodon post.
    ///
    /// ## Arguments
    ///
    /// * `document` - The HTML document to get the links from.
    /// * `tags` - The tags to ignore when getting links.
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

    /// Get the tags from the HTML content of a Mastodon post.
    ///
    /// ## Arguments
    ///
    /// * `document` - The HTML document to get the tags from.
    /// * `tags` - The tags to get from the document.
    fn get_tags(
        document: &dom_query::Document,
        tags: &Vec<megalodon::entities::status::Tag>
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut found_tags = Vec::new();

        for node in document.select("a").iter() {
            let href = node.attr("href").unwrap().to_string();

            // Check if the tag is in the list of tags to ignore.
            // We have to compare the lowercase versions of the URLs because
            // the Mastodon API returns the URLs in lowercase; whereas, the
            // HTML content may have the URLs in mixed case. This ensures that
            // the BlueSky post will be consistent with the Mastodon post's
            // formatting.
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
