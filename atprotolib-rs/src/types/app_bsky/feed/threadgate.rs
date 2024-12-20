use serde::{Deserialize, Serialize};

/*
    app.bsky.feed.threadgate
*/

/*    Type: mentionRule
    Id: app.bsky.feed.threadgate#mentionRule
    Kind: object

    Properties:
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct MentionRule {}

/*    Type: followingRule
    Id: app.bsky.feed.threadgate#followingRule
    Kind: object

    Properties:
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct FollowingRule {}

/*    Type: listRule
    Id: app.bsky.feed.threadgate#listRule
    Kind: object

    Properties:
    - list: string (JsonProperty: list) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct ListRule {
    #[serde(rename = "list")]
    pub list: String
}
