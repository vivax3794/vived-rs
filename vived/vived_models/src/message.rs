//! Guilded messages are like the text stuff

use serde::Deserialize;

/// The type of message
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    /// Your normal everyday message
    Default,
    /// A message issued by guilded themselves :O
    System,
}

impl MessageType {
    /// Returns `true` if the type is [`Default`].
    ///
    /// [`Default`]: Type::Default
    #[must_use]
    pub fn is_default(&self) -> bool {
        matches!(self, &Self::Default)
    }

    /// Returns `true` if the type is [`System`].
    ///
    /// [`System`]: Type::System
    #[must_use]
    pub fn is_system(&self) -> bool {
        matches!(self, &Self::System)
    }
}

/// Who was mentioned in a message
#[readonly::make]
#[derive(Deserialize, Debug, Default, Clone)]
#[serde(default)]
pub struct Mentions {
    /// What users were mentioned
    pub users: Vec<crate::UserId>,
    /// What channels are mentioned
    pub channels: Vec<crate::ChannelId>,
    /// What roles are mentioned
    pub roles: Vec<crate::RoleId>,
    /// Did this message mention @everyone?
    pub everyone: bool,
    /// Did this message mention @here
    pub here: bool,
}


/// Fields used by the api to represent who created a message
/// They use 2 redundant fields
/// 
/// There is a more convenient structure the [`CreatedBy`] enum
/// You can translate this into that using `.into()` which gives you a nicer interface
/// 
/// I did try to make this deserialize into that automatically,
/// but because of limitations on serde flatten we cant 
#[readonly::make]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatedByRawFields {
    /// What user created this message, for a webhook this is the static id "Ann6LewA"
    created_by: crate::UserId,
    /// Potential id of webhook that created message, if present ignore `created_by`
    created_by_webhook_id: Option<crate::WebhookId>,    
}

/// Who created this message?
#[derive(Deserialize, Debug, Clone)]
#[serde(from = "CreatedByRawFields")]
pub enum CreatedBy {
    /// Message was sent by a webhook
    Webhook(crate::WebhookId),
    /// Message was sent by a normal user
    User(crate::UserId),
}

impl From<CreatedByRawFields> for CreatedBy {
    fn from(raw: CreatedByRawFields) -> Self {
        if let Some(webhook_id) = raw.created_by_webhook_id {
            Self::Webhook(webhook_id)
        } else {
            Self::User(raw.created_by)
        }
    }
}

impl CreatedBy {
    /// Return the user id if this was sent by a user
    #[must_use]
    pub fn as_user(&self) -> Option<&crate::UserId> {
        if let &Self::User(ref v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Return the webhook id if this was sent by a webhook
    #[must_use]
    pub fn as_webhook(&self) -> Option<&crate::WebhookId> {
        if let &Self::Webhook(ref v) = self {
            Some(v)
        } else {
            None
        }
    }
}

/// A guilded message!
#[readonly::make]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// The id of this message
    pub id: crate::MessageId,
    /// What types of message is it?
    #[serde(rename = "type")]
    pub message_type: MessageType,
    /// Id of server it was sent in
    /// In the future this will be `None` in dms, but atm dms are not supported by the api
    /// meaning this field is always a `Some`
    pub server_id: Option<crate::ServerId>,
    /// Channel message was sent in
    pub channel_id: crate::ChannelId,
    /// Content of the message
    pub content: Option<String>,
    ///  Message embeds, currently only supports up to one embed,
    /// but it is still a list
    #[serde(default)]
    pub embeds: Vec<crate::Embed>,
    /// Message ids replied to
    /// if present will contain between 1 and 5 elements
    pub reply_message_ids: Option<Vec<crate::MessageId>>,
    /// If message is private only people mentioned or replied to can see it (and mods)
    #[serde(default)]
    pub is_private: bool,
    /// If it is silent would not ping users
    #[serde(default)]
    pub is_silent: bool,
    /// Describes who and what was mentioned in this message
    #[serde(default)]
    pub mentions: Mentions,
    /// When was this message sent?
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Who sent this message?
    #[serde(flatten)]
    pub created_by: CreatedByRawFields,
}