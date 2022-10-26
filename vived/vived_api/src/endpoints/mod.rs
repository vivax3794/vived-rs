//! Implement different guilded api endpoints

/// Base url of guilded api
const BASE_URL: &str = "https://www.guilded.gg/api/v1";

mod messages;
mod server;
mod channels;

pub use messages::*;
pub use server::*;
pub use channels::*;