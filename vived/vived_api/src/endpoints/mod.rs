//! Implement different guilded api endpoints

const BASE_URL: &str = "https://www.guilded.gg/api/v1";

mod messages;
mod server;

pub use messages::*;
pub use server::*;