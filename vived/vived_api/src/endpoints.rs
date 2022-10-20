//! Implement different guilded api endpoints

use serde::Deserialize;
use serde_json::json;
use vived_models::{ChannelId, Message};

use crate::Endpoint;

/// Base url of the guilded api endpoints
const BASE_URL: &str = "https://www.guilded.gg/api/v1";

/// Send a message
#[derive(Debug)]
pub struct MessageCreate {
    /// Content to send
    content: Option<String>,
    /// Channel to send in
    channel: ChannelId,
}

impl MessageCreate {
    /// Create a new message create instruction for the given channel.
    #[must_use]
    pub fn new(channel: ChannelId) -> Self {
        Self {
            channel,
            content: None,
        }
    }

    /// Send a message with the given content
    #[must_use]
    pub fn with_content(mut self, content: &str) -> Self {
        self.content = Some(content.to_owned());
        self
    }
}

impl Endpoint<Message> for MessageCreate {
    fn build(&self, client: &reqwest::Client) -> reqwest::RequestBuilder {
        client
            .post(format!(
                "{BASE_URL}/channels/{id}/messages",
                id = self.channel
            ))
            .json(&json!({
                "content": self.content
            }))
    }

    fn from_raw(raw: String) -> Result<Message, serde_json::Error> {
        /// Response from the message create endpoint
        #[derive(Deserialize, Debug)]
        struct MessageCreateResponse {
            /// Message that was created
            message: Message,
        }
        Ok(serde_json::from_str::<MessageCreateResponse>(&raw)?.message)
    }
}
