use serde::{Deserialize, Serialize};

use super::defs::PostView;

/*
    app.bsky.feed.getPosts
*/

/*    Type: response
    Id: app.bsky.feed.getPosts#response
    Kind: object

    Properties:
    - posts: app.bsky.feed.defs#postView[] (JsonProperty: posts) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetPostsResponse {
    #[serde(rename = "posts")]
    pub posts: Vec<PostView>
}
