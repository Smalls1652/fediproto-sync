use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents an invite code.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.server.defs#inviteCode")]
pub struct InviteCode {
    /// The invite code.
    #[serde(rename = "code")]
    pub code: String,

    /// The number of available uses.
    #[serde(rename = "available", default)]
    pub available: i32,

    /// Whether the invite code is disabled.
    #[serde(rename = "disabled", default)]
    pub disabled: bool,

    /// The account the invite code is for.
    #[serde(rename = "forAccount")]
    pub for_account: String,

    /// The account that created the invite code.
    #[serde(rename = "createdBy")]
    pub created_by: String,

    /// The date and time the invite code was created.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    /// The uses of the invite code.
    #[serde(rename = "uses")]
    pub uses: Vec<InviteCodeUse>
}

/// Represents an invite code use.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.server.defs#inviteCodeUse")]
pub struct InviteCodeUse {
    /// The account that used the invite code.
    #[serde(rename = "usedBy")]
    pub used_by: String,

    /// The date and time the invite code was used.
    #[serde(rename = "usedAt")]
    pub used_at: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DidDoc {
    #[serde(rename = "@context")]
    pub context: Vec<String>,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "alsoKnownAs")]
    pub also_known_as: Vec<String>,

    #[serde(rename = "verificationMethod")]
    pub verification_method: Vec<DidDocVerificationMethods>,

    #[serde(rename = "service")]
    pub service: Vec<DidDocServices>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum DidDocVerificationMethods {
    #[serde(rename = "Multikey")]
    Multikey(DidDocVerificationMethodMultiKey)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DidDocVerificationMethodMultiKey {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "controller")]
    pub controller: String,

    #[serde(rename = "publicKeyMultibase")]
    pub public_key_multibase: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum DidDocServices {
    #[serde(rename = "AtprotoPersonalDataServer")]
    AtprotoPersonalDataServer(DidDocServiceAtprotoPersonalDataServer)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DidDocServiceAtprotoPersonalDataServer {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String
}
