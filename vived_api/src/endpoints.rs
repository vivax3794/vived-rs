use serde_json::json;
use serde::Deserialize;
use vived_modals::{ChannelId, Message};

use crate::Endpoint;

const BASE_URL: &str = "https://www.guilded.gg/api/v1";

pub struct MessageCreate {
    content: Option<String>,
    channel: ChannelId,
}

impl MessageCreate {
    #[must_use]
    pub fn new(channel: ChannelId) -> Self {
        Self {
            channel,
            content: None,
        }
    }

    #[must_use]
    pub fn with_content(mut self, content: &str) -> Self {
        self.content = Some(content.to_owned());
        self
    }
}

#[derive(Deserialize, Debug)]
pub struct MessageCreateResponse {
    pub message: Message
}

impl std::ops::Deref for MessageCreateResponse {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

impl Endpoint<MessageCreateResponse> for MessageCreate {
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
}
