//! Endpoints for interacting with channels

use super::BASE_URL;

use serde::{Deserialize, Serialize};

/// Get a channel from an id
pub struct GetChannel(vived_models::ChannelId);

impl GetChannel {
    /// Create a new `GetChannel` instructions
    pub fn new(id: impl Into<vived_models::ChannelId>) -> Self {
        Self(id.into())
    }
}

impl crate::Endpoint<vived_models::Channel> for GetChannel {
    fn build(&self, client: &reqwest::Client) -> reqwest::RequestBuilder {
        client.get(
            format!("{BASE_URL}/channels/{}", self.0)
        )
    }

    fn from_raw(raw: &str) -> Result<vived_models::Channel, serde_json::Error> {
        #[derive(Deserialize)]
        /// Response from the server
        struct ChannelGetResponse {
            /// Actual channel data
            channel: vived_models::Channel,
        }
        serde_json::from_str::<ChannelGetResponse>(raw).map(|r| r.channel)
    }
}