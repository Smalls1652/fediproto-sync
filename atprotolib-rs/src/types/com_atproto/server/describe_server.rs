use serde::{Deserialize, Serialize};

/*
    com.atproto.server.describeServer
*/

/// Represents a server description response.
///
/// [`com.atproto.server.describeServer#responses`](https://docs.bsky.app/docs/api/com-atproto-server-describe-server#responses)
#[derive(Serialize, Deserialize, Debug)]
pub struct DescribeServerResponse {
    /// Whether the server requires an invite code to join.
    #[serde(rename = "inviteCodeRequired", default)]
    pub invite_code_required: bool,

    /// Whether the server requires phone verification.
    #[serde(rename = "phoneVerificationRequired", default)]
    pub phone_verification_required: bool,

    /// The available user domains to use for handles.
    #[serde(rename = "availableUserDomains")]
    pub available_user_domains: Vec<String>,

    /// Links to the server's privacy policy and terms of service.
    #[serde(rename = "links")]
    pub links: DescribeServerResponseLinks,

    /// Contact information for the server.
    #[serde(rename = "contact")]
    pub contact: DescribeServerResponseContact
}

/// Represents links to the server's privacy policy and terms of service.
#[derive(Serialize, Deserialize, Debug)]
pub struct DescribeServerResponseLinks {
    /// A link to the server's privacy policy.
    #[serde(rename = "privacyPolicy", skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<String>,

    /// A link to the server's terms of service.
    #[serde(rename = "termsOfService", skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>
}

/// Represents contact information for the server.
#[derive(Serialize, Deserialize, Debug)]
pub struct DescribeServerResponseContact {
    /// The email address for the contact.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>
}
