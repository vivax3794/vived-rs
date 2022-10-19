use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum MessageType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "system")]
    System,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub server_id: Option<crate::ServerId>,
    pub channel_id: crate::ChannelId,
    pub content: String,
}
