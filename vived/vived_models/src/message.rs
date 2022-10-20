//! Guilded messages are like the text stuff

use serde::Deserialize;

/// The type of message
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    /// Your normal everyday message
    Default,
    /// A message issued by guilded themself :O
    System,
}

/// A guilded message!
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// What types of message is it?
    #[serde(rename = "type")]
    pub message_type: Type,
    /// Id of server it was sent in
    /// In the future this will be `None` in dms, but atm dms are not supported by the api
    /// meaning this field is alwas a `Some`
    pub server_id: Option<crate::ServerId>,
    /// Channel message was sent in
    pub channel_id: crate::ChannelId,
    /// Content of the message
    pub content: Option<String>,
}
