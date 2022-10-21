//! <https://www.guilded.gg/docs/api/chat/ChatMessage>

use serde::{Deserialize, Serialize};
use vived_models::{ChannelId, MessageId, Embed, Message};

use crate::Endpoint;

/// Base url of the guilded api endpoints
const BASE_URL: &str = "https://www.guilded.gg/api/v1";

// TODO: implement embed, private, silent, and reply_message_ids


/// Arguments passed as json to the guilded api
#[derive(Serialize, Default)]
pub struct MessageCreateArguments {
    /// Content to send
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    /// Embeds to send
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<Vec<Embed>>,
    /// Whether to send the message as a private message
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isPrivate")]
    private: Option<bool>,
    /// Whether to send the message silently
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isSilent")]
    silent: Option<bool>,
    /// Message ids to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_message_ids: Option<Vec<vived_models::MessageId>>,
}

/// Send a message
#[must_use]
pub struct MessageCreate {
    /// Channel to send in
    channel: ChannelId,
    /// Json arguments
    arguments: MessageCreateArguments,
}

impl MessageCreate {
    // we make two constructors so we can enforce that at least embed or content is given
    // you can add both by adding the missing one with the corresponding method

    /// Create a new message create instruction for the given channel.
    /// With the given content
    pub fn new_with_content(channel: impl Into<ChannelId>, content: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            arguments: MessageCreateArguments {
                content: Some(content.into()),
                ..Default::default()
            },
        }
    }

    /// Create a new message create instruction for the given channel.
    /// With the given embed
    pub fn new_with_embed(channel: impl Into<ChannelId>, embed: impl Into<Embed>) -> Self {
        Self {
            channel: channel.into(),
            arguments: MessageCreateArguments {
                embeds: Some(vec![embed.into()]),
                ..Default::default()
            },
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


/// Json arguments for `ChannelGetMessages`
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ChannelGetMessagesArguments {
    /// before
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<chrono::DateTime<chrono::Utc>>,
    /// after
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<chrono::DateTime<chrono::Utc>>,
    /// limit, defaults to 50, max 100
    limit: u8,
    /// include private messages, default false
    include_private: bool,
}

impl Default for ChannelGetMessagesArguments {
    fn default() -> Self {
        Self {
            before: None,
            after: None,
            limit: 50,
            include_private: false,
        }
    }
}

/// Get a list of recent messages in a channel
#[must_use]
pub struct ChannelGetMessages {
    /// Channel to get messages from
    channel: ChannelId,
    /// Arguments
    arguments: ChannelGetMessagesArguments,
}

impl ChannelGetMessages {
    /// Create a new `ChannelGetMessages` instruction for the given channel
    pub fn new(channel: impl Into<ChannelId>) -> Self {
        Self {
            channel: channel.into(),
            arguments: ChannelGetMessagesArguments::default(),
        }
    }

    /// Set the before argument
    pub fn before(mut self, before: chrono::DateTime<chrono::Utc>) -> Self {
        self.arguments.before = Some(before);
        self
    }

    /// Set the after argument
    pub fn after(mut self, after: chrono::DateTime<chrono::Utc>) -> Self {
        self.arguments.after = Some(after);
        self
    }

    /// Set the limit argument
    pub fn limit(mut self, limit: u8) -> Self {
        // limit is capped at 100
        // produce warning if limit is higher than 100
        if limit > 100 {
            log::warn!("limit is capped at 100, but {} was given", limit);
        }
        self.arguments.limit = limit.min(100); 

        self
    }

    /// Set the include private argument
    pub fn include_private(mut self, include_private: bool) -> Self {
        self.arguments.include_private = include_private;
        self
    }
}


impl Endpoint<Vec<Message>> for ChannelGetMessages {
    fn build(&self, client: &reqwest::Client) -> reqwest::RequestBuilder {
        client
            .get(format!(
                "{BASE_URL}/channels/{id}/messages",
                id = self.channel
            ))
            .query(&self.arguments)
    }

    /// # Errors
    /// - if the json is invalid or doesn't match the schema
    fn from_raw(raw: &str) -> Result<Vec<Message>, serde_json::Error> {
        /// Response from the channel get messages endpoint
        #[derive(Deserialize, Debug)]
        struct ChannelGetMessagesResponse {
            /// Messages
            messages: Vec<Message>,
        }
        serde_json::from_str::<ChannelGetMessagesResponse>(raw).map(|resp| resp.messages)
    }
}


/// Get specific message in a channel
#[must_use]
pub struct ChannelGetMessage {
    /// Channel to get message from
    channel: ChannelId,
    /// Message to get
    message: MessageId,
}

impl ChannelGetMessage {
    /// Create a new `ChannelGetMessage` instruction for the given channel and message
    pub fn new(channel: impl Into<ChannelId>, message: impl Into<MessageId>) -> Self {
        Self {
            channel: channel.into(),
            message: message.into(),
        }
    }
}

impl Endpoint<Message> for ChannelGetMessage {
    fn build(&self, client: &reqwest::Client) -> reqwest::RequestBuilder {
        client.get(format!(
            "{BASE_URL}/channels/{channel}/messages/{message}",
            channel = self.channel,
            message = self.message
        ))
    }

    /// # Errors
    /// - if the json is invalid or doesn't match the schema
    fn from_raw(raw: &str) -> Result<Message, serde_json::Error> {
        /// Response from the channel get message endpoint
        #[derive(Deserialize, Debug)]
        struct ChannelGetMessageResponse {
            /// Message
            message: Message,
        }
        serde_json::from_str::<ChannelGetMessageResponse>(raw).map(|resp| resp.message)
    }
}

/// Edit message json arguments
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct MessageEditArguments {
    /// Message content
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    /// Embeds
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<Vec<Embed>>,
}


/// Edit a message
#[must_use]
pub struct MessageEdit {
    /// Channel to edit message in
    channel: ChannelId,
    /// Message to edit
    message: MessageId,
    /// Arguments
    arguments: MessageEditArguments,
}

impl MessageEdit {
    /// Create a new `MessageEdit` instruction for the given channel and message
    pub fn new(channel: impl Into<ChannelId>, message: impl Into<MessageId>) -> Self {
        Self {
            channel: channel.into(),
            message: message.into(),
            arguments: MessageEditArguments::default(),
        }
    }

    /// Set the content argument
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.arguments.content = Some(content.into());
        self
    }

    /// Set the new embed, overwriting the old one
    pub fn embed(mut self, embed: Embed) -> Self {
        self.arguments.embeds = Some(vec![embed]);
        self
    }
}

impl Endpoint<Message> for MessageEdit {
    fn build(&self, client: &reqwest::Client) -> reqwest::RequestBuilder {
        client
            .put(format!(
                "{BASE_URL}/channels/{channel}/messages/{message}",
                channel = self.channel,
                message = self.message
            ))
            .json(&self.arguments)
    }

    /// # Errors
    /// - if the json is invalid or doesn't match the schema
    fn from_raw(raw: &str) -> Result<Message, serde_json::Error> {
        /// Response from the message edit endpoint
        #[derive(Deserialize, Debug)]
        struct MessageEditResponse {
            /// Message
            message: Message,
        }
        serde_json::from_str::<MessageEditResponse>(raw).map(|resp| resp.message)
    }
}

/// Delete a message
#[derive(Debug)]
#[must_use]
pub struct MessageDelete {
    /// Channel to delete message in
    channel: ChannelId,
    /// Message to delete
    message: MessageId,
}

impl MessageDelete {
    /// Create a new `MessageDelete` instruction for the given channel and message
    pub fn new(channel: impl Into<ChannelId>, message: impl Into<MessageId>) -> Self {
        Self {
            channel: channel.into(),
            message: message.into(),
        }
    }
}

impl Endpoint<()> for MessageDelete {
    fn build(&self, client: &reqwest::Client) -> reqwest::RequestBuilder {
        client.delete(format!(
            "{BASE_URL}/channels/{channel}/messages/{message}",
            channel = self.channel,
            message = self.message
        ))
    }

    /// # Errors
    /// - if the json is invalid or doesn't match the schema
    fn from_raw(_: &str) -> Result<(), serde_json::Error> {
        Ok(())
    }
}