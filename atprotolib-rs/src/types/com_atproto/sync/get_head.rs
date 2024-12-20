use serde::{Deserialize, Serialize};

/*
    com.atproto.sync.getHead
*/

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetHeadResponse {
    #[serde(rename = "root")]
    pub root: String
}
