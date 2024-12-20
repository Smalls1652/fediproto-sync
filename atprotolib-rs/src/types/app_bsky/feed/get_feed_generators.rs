use serde::{Deserialize, Serialize};

use super::defs::GeneratorView;

/*
    app.bsky.feed.getFeedGenerators
*/

/*    Type: response
    Id: app.bsky.feed.getFeedGenerators#response
    Kind: object

    Properties:
    - feeds: app.bsky.feed.defs#generatorView[] (JsonProperty: feeds) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetFeedGeneratorsResponse {
    #[serde(rename = "feeds")]
    pub feeds: Vec<GeneratorView>
}
