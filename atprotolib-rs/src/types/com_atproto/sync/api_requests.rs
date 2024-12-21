use serde::{Deserialize, Serialize};

/// Represents a request to notify of an update.
#[derive(Serialize, Deserialize, Debug)]
pub struct NotifyOfUpdateRequest {
    /// The hostname of the server.
    #[serde(rename = "hostname")]
    pub hostname: String
}

/// Represents a request to crawl a hostname.
#[derive(Serialize, Deserialize, Debug)]
pub struct CrawlRequest {
    /// The hostname to crawl.
    #[serde(rename = "hostname")]
    pub hostname: String
}
