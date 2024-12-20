use serde::{Deserialize, Serialize};

use super::{defs::ThreadgateView, BlockedPost, NotFoundPost, ThreadViewPost};

/*
    app.bsky.feed.getPostThread
*/

/*    Type: response
    Id: app.bsky.feed.getPostThread#response
    Kind: object

    Properties:
    - thread: union  (JsonProperty: thread) [Required]
    - threadgate: app.bsky.feed.defs#threadgateView (JsonProperty: threadgate) [Optional]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct GetPostThreadResponse {
    #[serde(rename = "thread")]
    pub thread: GetPostThreadResponseThread,
    #[serde(rename = "threadgate", skip_serializing_if = "Option::is_none")]
    pub threadgate: Option<ThreadgateView>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetPostThreadResponseThread {
    ThreadView(ThreadViewPost),
    NotFound(NotFoundPost),
    Blocked(BlockedPost)
}
