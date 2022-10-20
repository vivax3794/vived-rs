//! Ids are used to identify specific resources on Guilded


// We don't really need docs for each specific id
#![allow(missing_docs)]

use serde::Deserialize;

/// Define the ids used in the guilded api
/// They all consist of strings 
macro_rules! define_id {
    (pub struct  $id:ident($ty:path)) => {
            #[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
            #[serde(transparent)]
            pub struct $id($ty);

            impl ::std::fmt::Display for $id {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl ::std::convert::From<$ty> for $id {
                fn from(id: $ty) -> Self {
                    Self(id)
                }
            }
    };
}

define_id!(pub struct ServerId(String));
define_id!(pub struct ChannelId(String));
define_id!(pub struct MessageId(String));
define_id!(pub struct UserId(String));
define_id!(pub struct WebhookId(String));
define_id!(pub struct RoleId(usize));

// We cant do this in the macro as not every id can implement copy
// so we just mark it as copy here
impl Copy for RoleId {}