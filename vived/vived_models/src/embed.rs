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

impl From<String> for EmbedFooter {
    fn from(text: String) -> Self {
        Self {
            icon_url: None,
            text,
        }
    }
}

impl From<&str> for EmbedFooter {
    fn from(text: &str) -> Self {
        Self {
            icon_url: None,
            text: text.to_owned(),
        }
    }
}

/// Embed Thumbnail, this is just a url
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct EmbedImage {
    /// Url of the thumbnail
    pub url: String,
}

// lets make it convenient to construct an embed image and grab the string
impl From<String> for EmbedImage {
    fn from(url: String) -> Self {
        Self { url }
    }
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The description of the embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The url of the embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The color of the embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::Color>,
    /// The footer of the embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    /// The timestamp to put in the footer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,

    /// Thumbnail of the embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedImage>,
    /// Image of the embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,

    /// Embed Author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    /// Fields of the embed
    #[serde(default)]
    pub fields: Vec<EmbedField>,
}


// Implement builder pattern for embed
impl Embed {
    /// Create a new embed
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the title of the embed
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the description of the embed
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the url of the embed
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the color of the embed
    #[must_use]
    pub fn color(mut self, color: crate::Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the footer of the embed
    #[must_use]
    pub fn footer(mut self, footer: impl Into<EmbedFooter>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Set the timestamp of the footer
    #[must_use]
    pub fn timestamp(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// Set the thumbnail of the embed
    #[must_use]
    pub fn thumbnail(mut self, thumbnail: impl Into<EmbedImage>) -> Self {
        self.thumbnail = Some(thumbnail.into());
        self
    }

    /// Set the image of the embed
    #[must_use]
    pub fn image(mut self, image: impl Into<EmbedImage>) -> Self {
        self.image = Some(image.into());
        self
    }

    /// Set the author of the embed
    #[must_use]
    pub fn author(mut self, author: EmbedAuthor) -> Self {
        self.author = Some(author);
        self
    }

    /// Add a field to the embed
    #[must_use]
    pub fn field(mut self, field: EmbedField) -> Self {
        self.fields.push(field);
        self
    }

}