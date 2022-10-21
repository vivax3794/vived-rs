//! Ids are used to identify specific resources on Guilded


// We don't really need docs for each specific id
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Define the ids used in the guilded api
/// They all consist of strings 
macro_rules! define_string_id {
    (pub struct  $id:ident(String)) => {
            #[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
            #[serde(transparent)]
            pub struct $id(pub String);

            impl ::std::fmt::Display for $id {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl ::std::convert::From<String> for $id {
                fn from(id: String) -> Self {
                    Self(id)
                }
            }
            
            impl ::std::convert::From<&str> for $id {
                fn from(id: &str) -> Self {
                    Self(id.to_owned())
                }
            }
    };
}

define_string_id!(pub struct ServerId(String));
define_string_id!(pub struct ChannelId(String));
define_string_id!(pub struct MessageId(String));
define_string_id!(pub struct UserId(String));
define_string_id!(pub struct WebhookId(String));

// For some reason RoleId uses a `usize` instead of a String
// So we need to special case it

/// A role id
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[serde(transparent)]
pub struct RoleId(pub usize);

impl ::std::fmt::Display for RoleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ::std::convert::From<usize> for RoleId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}