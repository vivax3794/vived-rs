//! Guilded websocket events.

use serde::Deserialize;

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
}
