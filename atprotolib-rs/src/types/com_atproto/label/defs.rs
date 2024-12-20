use serde::{Deserialize, Serialize};

/// Metadata tag on an atproto resource (eg, repo or record).
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.label.defs#label")]
pub struct Label {
    /// The AT Protocol version of the label object.
    #[serde(rename = "ver", default)]
    pub ver: i32,

    /// DID of the actor who created this label.
    #[serde(rename = "src")]
    pub src: String,

    /// AT URI of the record, repository (account), or other resource that this label applies to.
    #[serde(rename = "uri")]
    pub uri: String,

    /// CID specifying the specific version of 'uri' resource this label applies to.
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,

    /// The short string name of the value or type of this label.
    #[serde(rename = "val")]
    pub val: String,

    /// Whether this label negates another label.
    #[serde(rename = "neg", default)]
    pub neg: bool,

    /// Date and time the label was created.
    #[serde(rename = "cts")]
    pub cts: String,

    /// Date and time the label will expire.
    #[serde(rename = "exp", skip_serializing_if = "Option::is_none")]
    pub exp: Option<String>,

    /// Signature of dag-cbor encoded label.
    #[serde(rename = "sig", skip_serializing_if = "Option::is_none")]
    pub sig: Option<Vec<u8>>
}

/// Metadata tags on an atproto record, published by the author within the record.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "$type", rename = "com.atproto.label.defs#selfLabels")]
pub struct SelfLabels {
    /// A list of self labels.
    #[serde(rename = "values")]
    pub values: Vec<SelfLabel>
}

/// Metadata tag on an atproto record, published by the author within the record.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "$type", rename = "com.atproto.label.defs#selfLabel")]
pub struct SelfLabel {
    /// The short string name of the value or type of this label.
    #[serde(rename = "val")]
    pub val: String
}

/// Declares a label value and its expected interpretations and behaviors
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type", rename = "com.atproto.label.defs#labelValueDefinition")]
pub struct LabelValueDefinition {
    /// The value of the label being defined. Must only include lowercase ascii and the '-' character ([a-z-]+).
    #[serde(rename = "identifier")]
    pub identifier: String,

    /// How should a client visually convey this label? 'inform' means neutral and informational; 'alert' means negative and warning; 'none' means show nothing.
    #[serde(rename = "severity")]
    pub severity: String,

    /// What should this label hide in the UI, if applied? 'content' hides all of the target; 'media' hides the images/video/audio; 'none' hides nothing.
    #[serde(rename = "blurs")]
    pub blurs: String,

    /// The default setting for this label.
    #[serde(rename = "defaultSetting", skip_serializing_if = "Option::is_none")]
    pub default_setting: Option<String>,

    /// Does the user need to have adult content enabled in order to configure this label?
    #[serde(rename = "adultOnly", default)]
    pub adult_only: bool,

    /// Locale specific display strings for the label.
    #[serde(rename = "locales")]
    pub locales: Vec<LabelValueDefinitionStrings>
}

/// Strings which describe the label in the UI, localized into a specific language.
#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "$type",
    rename = "com.atproto.label.defs#labelValueDefinitionStrings"
)]
pub struct LabelValueDefinitionStrings {
    /// The language code of the language these strings are written in.
    #[serde(rename = "lang")]
    pub lang: String,

    /// A short human-readable name for the label.
    #[serde(rename = "name")]
    pub name: String,

    /// A longer description of what the label means and why it might be applied.
    #[serde(rename = "description")]
    pub description: String
}
