//! <https://www.guilded.gg/docs/api/chat/ChatMessage>

use serde::{Deserialize, Serialize};
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
        /// Arguments passed as json to the guilded api
        #[derive(Serialize)]
        pub struct MessageCreateArguments {
            /// Content to send
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<String>,
            /// Embeds to send
            #[serde(skip_serializing_if = "Option::is_none")]
            embeds: Option<Vec<Embed>>,
        }
        client
            .post(format!(
                "{BASE_URL}/channels/{id}/messages",
                id = self.channel
            ))
            .json(&MessageCreateArguments {
                content: self.content.clone(),
                embeds: self.embeds.clone(),
            })
    }

    /// # Errors
    /// - if the json is invalid or doesn't match the schema
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
