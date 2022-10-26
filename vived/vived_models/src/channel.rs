//! Guilded channels
//! <https://www.guilded.gg/docs/api/channels/Mentions>

use serde::Deserialize;

/// Channel type
#[non_exhaustive]
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ChannelType {
    /// Announcements
    Announcements,
    /// A normal text channel
    Chat,
    /// Calender channel
    Calendar,
    /// Forum channel
    Forums,
    /// Media channel
    Media,
    /// Docs channel
    Docs,
    /// Voice Channel
    Voice,
    /// List channel
    List,
    /// Schedule channel
    Scheduling,
    /// Stream channel
    Stream,
}

/// Thread Archived Information 
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ThreadArchivedInfo {
    /// Archived at timestamp
    pub archived_at: chrono::DateTime<chrono::Utc>,
    /// Archived by user id
    pub archived_by: crate::UserId,
}

/// Channel information
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    /// The id of the channel
    pub id: crate::ChannelId,
    /// The type of the channel
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    /// The name of the channel
    pub name: String,
    /// The topic of the channel
    pub topic: Option<String>,
    /// Created at timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Created by
    pub created_by: crate::UserId,
    /// Updated at
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// server id
    pub server_id: crate::ServerId,
    /// parent id
    pub parent_id: Option<crate::ChannelId>,
    /// category id
    pub category_id: Option<crate::ChannelId>,
    /// group id
    pub group_id: Option<crate::GroupId>,
    /// is public
    #[serde(default)]
    pub is_public: bool,
    /// Archived information
    #[serde(flatten)]
    pub archived_info: Option<ThreadArchivedInfo>,
}