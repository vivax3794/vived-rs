use serde::Deserialize;

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

pub struct TestId(String);

define_ids!(
    ServerId, ChannelId
);
