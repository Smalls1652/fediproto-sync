use serde::{Deserialize, Serialize};

/// Represents a response to get recommended DID credentials.
#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendedDidCredentialsResponse {
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

/// Represents a response to a request to resolve a handle to a DID.
#[derive(Serialize, Deserialize, Debug)]
pub struct ResolveHandleResponse {
    /// The DID associated with the handle.
    #[serde(rename = "did")]
    pub did: String
}

/// Represents a response to sign a PLC operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct SignPlcOperationResponse {
    /// A value of the operation. (?)
    #[serde(rename = "operation")]
    pub operation: serde_json::Value
}
