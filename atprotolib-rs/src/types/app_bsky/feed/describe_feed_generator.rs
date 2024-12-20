use serde::{Deserialize, Serialize};

/*
    app.bsky.feed.describeFeedGenerator
*/

/*    Type: response
    Id: app.bsky.feed.describeFeedGenerator#response
    Kind: object

    Properties:
    - did: string (JsonProperty: did) [Required]
    - feeds: #feed[] (JsonProperty: feeds) [Required]
    - links: #links (JsonProperty: links) [Optional]
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DescribeFeedGeneratorResponse {
    did: String,
    feeds: Vec<Feed>,
    links: Option<Links>
}

/*    Type: feed
    Id: app.bsky.feed.describeFeedGenerator#feed
    Kind: object

    Properties:
    - uri: string (JsonProperty: uri) [Required]
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Feed {
    uri: String
}

/*    Type: links
    Id: app.bsky.feed.describeFeedGenerator#links
    Kind: object

    Properties:
    - privacy_policy: string (JsonProperty: privacyPolicy) [Optional]
    - terms_of_service: string (JsonProperty: termsOfService) [Optional]
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    privacy_policy: Option<String>,
    terms_of_service: Option<String>
}
