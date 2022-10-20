//! Ids are used to identify specific resources on Guilded


// We dont really need docs for each specific id
#![allow(missing_docs)]

use serde::Deserialize;

/// Define the ids used in the guilded api
/// They all consist of strings 
macro_rules! define_ids {
    ($($id:ident),*) => {
        $(
            #[derive(Debug, Deserialize, Eq, PartialEq)]
            #[serde(transparent)]
            pub struct $id(String);

            impl ::std::fmt::Display for $id {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl ::std::convert::From<&str> for $id {
                fn from(id: &str) -> Self {
                    Self(id.to_owned())
                }
            }
        )*
    };
}

define_ids!(
    ServerId,
    ChannelId
);
