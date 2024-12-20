use serde::{Deserialize, Serialize};

/*
    com.atproto.identity.submitPlcOperation
*/

/// Represents a request to submit a PLC operation.
#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitPlcOperationRequest {
    /// The operation to submit.
    #[serde(rename = "operation")]
    pub operation: serde_json::Value
}
