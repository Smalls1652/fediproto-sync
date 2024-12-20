use serde::{Deserialize, Serialize};

use super::Label;

/*
    com.atproto.label.subscribeLabels
*/

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Labels {
    #[serde(rename = "seq")]
    pub seq: i64,
    #[serde(rename = "labels")]
    pub labels: Vec<Label>
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>
}
