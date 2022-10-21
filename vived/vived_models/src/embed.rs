//! Embeds are nice features that allow you to send much nicer formatted text

use serde::{Deserialize, Serialize};

/// Footer of an embed
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct EmbedFooter {
    /// Icon of the footer
    pub icon_url: Option<String>,
    /// Text of the footer
    pub text: String,
}

/// Embed Thumbnail, this is just a url
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct EmbedImage {
    /// Url of the thumbnail
    pub url: String,
}

// lets make it convenient to construct an embed image and grab the string
impl From<&str> for EmbedImage {
    fn from(v: &str) -> Self {
        Self { url: v.to_owned() }
    }
}

// lets make this just return the internal url
impl std::fmt::Display for EmbedImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.url.fmt(f)
    }
}

/// Embed Author
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct EmbedAuthor {
    /// Name of the author
    pub name: String,
    /// Url of the author
    pub url: Option<String>,
    /// Icon of the author
    pub icon_url: Option<String>,
}

/// Embed field
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct EmbedField {
    /// Name of the field
    pub name: String,
    /// Value of the field
    pub value: String,
    /// Whether or not this field should be inline
    pub inline: bool,
}

/// A guilded embed
///
/// Recommended way of creating an embed is using the struct default syntax like this:
/// ```rust
/// let embed = Embed {
///     title: Some("hello world".to_owned()),
///     ..default()
/// };
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    /// The title of the embed
    pub title: Option<String>,
    /// The description of the embed
    pub description: Option<String>,
    /// The url of the embed
    pub url: Option<String>,
    /// The color of the embed
    pub color: Option<crate::Color>,
    /// The footer of the embed
    pub footer: Option<EmbedFooter>,
    /// The timestamp to put in the footer
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,

    /// Thumbnail of the embed
    pub thumbnail: Option<EmbedImage>,
    /// Image of the embed
    pub image: Option<EmbedImage>,

    /// Embed Author
    pub author: Option<EmbedAuthor>,
    /// Fields of the embed
    #[serde(default)]
    pub fields: Vec<EmbedField>,
}
