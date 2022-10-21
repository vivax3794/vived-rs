//! <https://www.guilded.gg/docs/api/chat/ChatMessage>

use serde::{Deserialize, Serialize};
use vived_models::{ChannelId, Embed, Message};

use crate::Endpoint;

/// Base url of the guilded api endpoints
const BASE_URL: &str = "https://www.guilded.gg/api/v1";

// TODO: implement embed, private, silent, and reply_message_ids

/// Arguments passed as json to the guilded api
#[derive(Serialize, Debug, Clone, Default)]
pub struct MessageCreateArguments {
    /// Content to send
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    /// Embeds to send
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<Vec<Embed>>,
    /// Whether to send the message as a private message
    #[serde(skip_serializing_if = "Option::is_none")]
    private: Option<bool>,
    /// Whether to send the message silently
    #[serde(skip_serializing_if = "Option::is_none")]
    silent: Option<bool>,
    /// Message ids to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_message_ids: Option<Vec<vived_models::MessageId>>,
}

/// Send a message
#[derive(Debug)]
#[must_use]
pub struct MessageCreate {
    /// Channel to send in
    channel: ChannelId,
    /// Json arguments
    arguments: MessageCreateArguments,
}

impl MessageCreate {
    /// Create a new message create instruction for the given channel.
    pub fn new<I: Into<ChannelId>>(channel: I) -> Self {
        Self {
            channel: channel.into(),
            arguments: MessageCreateArguments::default(),
        }
    }

    // implement builder pattern for the MessageCreateArguments

    /// Set the content of the message
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.arguments.content = Some(content.into());
        self
    }

    /// Embed to send
    pub fn embed(mut self, embed: Embed) -> Self {
        self.arguments.embeds = Some(vec![embed]);
        self
    }

    /// Is Private
    pub fn private(mut self, private: bool) -> Self {
        self.arguments.private = Some(private);
        self
    }

    /// Is Silent
    pub fn silent(mut self, silent: bool) -> Self {
        self.arguments.silent = Some(silent);
        self
    }

    /// Reply Message Ids
    pub fn replies(mut self, replies: Vec<impl Into<vived_models::MessageId>>) -> Self {
        self.arguments.reply_message_ids = Some(replies.into_iter().map(Into::into).collect());
        self
    }

    /// Add single reply
    pub fn reply(mut self, reply: impl Into<vived_models::MessageId>) -> Self {
        self.arguments
            .reply_message_ids
            .get_or_insert_with(Vec::new)
            .push(reply.into());

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
            .json(&self.arguments)
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
