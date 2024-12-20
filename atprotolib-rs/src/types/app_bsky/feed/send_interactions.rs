use serde::{Deserialize, Serialize};

use super::defs::Interaction;

/*
    app.bsky.feed.sendInteractions
*/

/*    Type: request
    Id: app.bsky.feed.sendInteractions#request
    Kind: object

    Properties:
    - interactions: app.bsky.feed.defs#interaction[] (JsonProperty: interactions) [Required]
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct SendInteractionsRequest {
    #[serde(rename = "interactions")]
    pub interactions: Vec<Interaction>
}

/*    Type: response
    Id: app.bsky.feed.sendInteractions#response
    Kind: object

    Properties:
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct SendInteractionsResponse {}
