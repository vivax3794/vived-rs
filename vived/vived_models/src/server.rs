//! Information about guilded servers
//! <https://www.guilded.gg/docs/api/servers/Server>

use serde::Deserialize;

/// The type of the server
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ServerType {
    /// A Team server
    Team,
    /// A organization server
    Organization,
    /// A community server
    Community,
    /// a clan server
    Clan,
    /// A guild server
    Guild,
    /// A friends server
    Friends,
    /// A streaming server
    Streaming,
    /// Other server type
    Other,
}

/// Information about a guilded server
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    /// The id of the server
    pub id: crate::ServerId,
    /// The id of the server owner
    pub owner_id: crate::UserId,
    /// The type of the server
    pub server_type: Option<ServerType>,
    /// The name of the server
    pub name: String,
    /// The url part of the server
    pub url: String,
    /// The description of the server
    pub about: Option<String>,
    /// The avatar of the server
    /// A media-uri string
    pub avatar: Option<String>,
    /// The banner of the server
    /// A media-uri string
    pub banner: Option<String>,
    /// The timezone of the server
    pub timezone: Option<String>,
    /// The verified status of the server
    #[serde(default)]
    pub verified: bool,
    /// Channel id of the default channel
    pub default_channel_id: Option<crate::ChannelId>,
    /// Created at timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Server {
    /// Get the url of the server
    #[must_use]
    pub fn url(&self) -> String {
        format!("https://www.guilded.gg/{}", self.url)
    }
}

impl From<Server> for crate::ServerId {
    fn from(server: Server) -> Self {
        server.id
    }
}