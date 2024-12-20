use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.requestCrawl
*/

/// Represents a request to crawl a hostname.
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCrawlRequest {
    /// The hostname to crawl.
    #[serde(rename = "hostname")]
    pub hostname: String
}
