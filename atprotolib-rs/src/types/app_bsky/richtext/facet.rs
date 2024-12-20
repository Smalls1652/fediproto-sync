use serde::{Deserialize, Serialize};

/*
    app.bsky.richtext.facet
*/

/// Represents a facet of rich text.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "$type", rename = "app.bsky.richtext.facet")]
pub struct RichTextFacet {
    /// The index of the facet in the rich text.
    #[serde(rename = "index")]
    pub index: ByteSlice,

    /// The features of the facet.
    #[serde(rename = "features")]
    pub features: Vec<RichTextFacetFeature>
}

/// A type union for the features of a rich text facet.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "$type")]
pub enum RichTextFacetFeature {
    /// A mention.
    #[serde(rename = "app.bsky.richtext.facet#mention")]
    Mention(RichTextFacetMention),

    /// A link.
    #[serde(rename = "app.bsky.richtext.facet#link")]
    Link(RichTextFacetLink),

    /// A tag.
    #[serde(rename = "app.bsky.richtext.facet#tag")]
    Tag(RichTextFacetTag)
}

/// Represents a mention in a rich text facet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RichTextFacetMention {
    /// The DID of the mention.
    #[serde(rename = "did")]
    pub did: String
}

/// Represents a link in a rich text facet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RichTextFacetLink {
    /// The URI of the link.
    #[serde(rename = "uri")]
    pub uri: String
}

/// Represents a tag in a rich text facet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RichTextFacetTag {
    /// The tag.
    #[serde(rename = "tag")]
    pub tag: String
}

/// Represents a byte slice.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ByteSlice {
    /// The start byte.
    #[serde(rename = "byteStart")]
    pub byte_start: i64,

    /// The end byte.
    #[serde(rename = "byteEnd")]
    pub byte_end: i64
}
