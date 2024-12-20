use serde::{Deserialize, Serialize};

use super::defs::GeneratorView;

/*
    app.bsky.feed.getSuggestedFeeds
*/

/*    Type: response
    Id: app.bsky.feed.getSuggestedFeeds#response
    Kind: object

    Properties:
    - cursor: string (JsonProperty: cursor) [Optional]
    - feeds: app.bsky.feed.defs#generatorView[] (JsonProperty: feeds) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetSuggestedFeedsResponse {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "feeds")]
    pub feeds: Vec<GeneratorView>
}
