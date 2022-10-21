//! <https://www.guilded.gg/docs/api/chat/ChatMessage>

use serde::Deserialize;
use serde_json::json;
use vived_models::{ChannelId, Message, Embed};

use crate::Endpoint;

/// Base url of the guilded api endpoints
const BASE_URL: &str = "https://www.guilded.gg/api/v1";

// TODO: implement embed, private, silent, and reply_message_ids

/// Send a message
#[derive(Debug)]
#[must_use]
pub struct MessageCreate {
    /// Content to send
    content: Option<String>,
    /// Channel to send in
    channel: ChannelId,
    /// Embeds to send
    embeds: Option<Vec<Embed>>,
}

impl MessageCreate {
    /// Create a new message create instruction for the given channel.
    pub fn new<I: Into<ChannelId>>(channel: I) -> Self {
        Self {
            channel: channel.into(),
            content: None,
            embeds: None,
        }
    }

    /// Send a message with the given content
    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    /// Send a message with the given embeds
    /// Currently only supports one embed, in the future a `with_embeds` method will be added
    pub fn with_embed(mut self, embed: Embed) -> Self {
        self.embeds = Some(vec![embed]);
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
                "content": self.content,
                "embeds": self.embeds,
            }))
    }

    fn from_raw(raw: &str) -> Result<Message, serde_json::Error> {
        /// Response from the message create endpoint
        #[derive(Deserialize, Debug)]
        struct MessageCreateResponse {
            /// Message that was created
            message: Message,
        }
        serde_json::from_str::<MessageCreateResponse>(raw).map(|resp| resp.message)
    }
}
