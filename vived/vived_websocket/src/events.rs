//! Guilded websocket events.

use serde::Deserialize;

/// `MessageDeleteData` is the data for a message delete event.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageDeleteData {
    /// The id of the message that was deleted.
    pub id: vived_models::MessageId,
    /// The id of the server the message was deleted from.
    pub server_id: vived_models::ServerId,
    /// The id of the channel the message was deleted from.
    pub channel_id: vived_models::ChannelId,
    /// The time the message was deleted at
    pub deleted_at: chrono::DateTime<chrono::Utc>,
    /// Was message private
    pub is_private: bool,
}

/// A Guilded event.
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "t", content = "d")]
pub enum GuildedEvent {
    /// A message was created.
    ChatMessageCreated { 
        /// What server the message was created in.
        #[serde(rename = "serverId")]
        server_id: vived_models::ServerId,
        /// Message data.
        message: vived_models::Message
    },
    /// Chat message was updated.
    ChatMessageUpdated {
        /// What server the message was updated in.
        #[serde(rename = "serverId")]
        server_id: vived_models::ServerId,
        /// Message data.
        message: vived_models::Message
    },
    /// Chat message was deleted.
    ChatMessageDeleted {
        /// What server the message was deleted in.
        #[serde(rename = "serverId")]
        server_id: vived_models::ServerId,
        /// Message data.
        message: MessageDeleteData
    }, 
}
