//! Embeds are nice features that allow you to send much nicer formatted text

use serde::{Deserialize, Serialize};

/// Footer of an embed
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct EmbedFooter {
    /// Icon of the footer
    #[serde(default)]
    pub icon_url: Option<String>,
    /// Text of the footer
    pub text: String,
}

impl EmbedFooter {
    /// Set icon url
    #[must_use]
    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
}

// lets make it convenient to construct an embed image and grab the string
impl From<String> for EmbedImage {
    fn from(url: String) -> Self {
        Self { url: Some(url) }
    }
}

impl From<&str> for EmbedImage {
    fn from(v: &str) -> Self {
        Self { url: Some(v.to_owned()) }
    }
}


/// Embed Author
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct EmbedAuthor {
    /// Name of the author
    pub name: String,
    /// Url of the author
    #[serde(default)]
    pub url: Option<String>,
    /// Icon of the author
    #[serde(default)]
    pub icon_url: Option<String>,
}

impl EmbedAuthor {
    /// Set the url of the author
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the icon url of the author
    #[must_use]
    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }
}

impl From<String> for EmbedAuthor {
    fn from(name: String) -> Self {
        Self {
            name,
            url: None,
            icon_url: None,
        }
    }
}

impl From<&str> for EmbedAuthor {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            url: None,
            icon_url: None,
        }
    }
}

/// Embed field
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct EmbedField {
    /// Name of the field
    pub name: String,
    /// Value of the field
    pub value: String,
    /// Whether or not this field should be inline
    #[serde(default)]
    pub inline: bool,
}

impl EmbedField {
    /// Construct new embed field
    /// Name and text are required
    #[must_use]
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: text.into(),
            inline: false,
        }
    }

    /// Set whether or not this field should be inline
    #[must_use]
    pub fn inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }
}

/// A guilded embed
///
/// # Example of all embed fields
/// ```rust
/// vived::Embed::new()
///    .title("Hello world")
///    .description("This is a test message")
///    .color(0x00ff00)
///    .url("https://www.guilded.gg")
///    .timestamp(chrono::Utc::now())
///    .footer(vived::EmbedFooter::from("This is a footer").icon_url(
///        "https://img.guildedcdn.com/asset/DefaultUserAvatars/profile_1.png",
///    ))
///    .thumbnail("https://img.guildedcdn.com/asset/DefaultUserAvatars/profile_2.png")
///    .image("https://img.guildedcdn.com/asset/DefaultUserAvatars/profile_3.png")
///    .author(
///        vived::EmbedAuthor::from("This is an author")
///            .url("https://www.guilded.gg")
///            .icon_url(
///                "https://img.guildedcdn.com/asset/DefaultUserAvatars/profile_4.png",
///            ),
///    )
///    // lets create two of each
///    .field(vived::EmbedField::new("Field 1", "This is field 1"))
///    .field(vived::EmbedField::new("Field 2", "This is field 2"))
///    .field(vived::EmbedField::new("Field 3", "This is field 3").inline(true))
///    .field(vived::EmbedField::new("Field 4", "This is field 4").inline(true)),
/// ```
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
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
    #[serde(default)]
    pub thumbnail: EmbedImage,
    /// Image of the embed
    #[serde(default)]
    pub image: EmbedImage,

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
    pub fn color(mut self, color: impl Into<crate::Color>) -> Self {
        self.color = Some(color.into());
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
        self.thumbnail = thumbnail.into();
        self
    }

    /// Set the image of the embed
    #[must_use]
    pub fn image(mut self, image: impl Into<EmbedImage>) -> Self {
        self.image = image.into();
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