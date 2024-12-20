use serde::{Deserialize, Serialize};

/*
    com.atproto.identity.getRecommendedDidCredentials
*/

/// Represents a response to get recommended DID credentials.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetRecommendedDidCredentialsResponse {
    /// The rotation keys.
    #[serde(rename = "rotationKeys")]
    pub rotation_keys: Vec<String>,

    /// The also known as.
    #[serde(rename = "alsoKnownAs")]
    pub also_known_as: Vec<String>,

    /// The verification methods.
    #[serde(rename = "verificationMethods")]
    pub verification_methods: serde_json::Value,

    /// The services.
    #[serde(rename = "services")]
    pub services: serde_json::Value
}
