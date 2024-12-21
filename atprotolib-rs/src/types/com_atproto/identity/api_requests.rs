use serde::{Deserialize, Serialize};

/// Represents a request to sign a PLC operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct SignPlcOperationRequest {
    /// A token received through com.atproto.identity.requestPlcOperationSignature
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// The rotation keys.
    #[serde(rename = "rotationKeys", skip_serializing_if = "Option::is_none")]
    pub rotation_keys: Option<Vec<String>>,

    /// The also known as.
    #[serde(rename = "alsoKnownAs", skip_serializing_if = "Option::is_none")]
    pub also_known_as: Option<Vec<String>>,

    /// The verification methods.
    #[serde(
        rename = "verificationMethods",
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_methods: Option<serde_json::Value>,

    /// The services.
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<serde_json::Value>
}

/// Represents a request to submit a PLC operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitPlcOperationRequest {
    /// The operation to submit.
    #[serde(rename = "operation")]
    pub operation: serde_json::Value
}

/// Represents a request to update a handle.
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHandleRequest {
    /// The new handle.
    #[serde(rename = "handle")]
    pub handle: String
}
