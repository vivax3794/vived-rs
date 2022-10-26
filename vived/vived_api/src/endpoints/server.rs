//! Server related endpoints

use serde::Deserialize;

use super::BASE_URL;

/// Get a server by id
pub struct GetServer(vived_models::ServerId);

impl GetServer {
    /// Create a new `GetServer` instructions
    pub fn new(id: impl Into<vived_models::ServerId>) -> Self {
        Self(id.into())
    }
}

impl crate::Endpoint<vived_models::Server> for GetServer {
    fn build(&self, client: &reqwest::Client) -> reqwest::RequestBuilder {
        client.get(
            format!("{BASE_URL}/servers/{}", self.0)
        )
    }

    fn from_raw(raw: &str) -> Result<vived_models::Server, serde_json::Error> {
        #[derive(Deserialize)]
        /// Response from the server
        struct ServerGetResponse {
            /// Actual server data
            server: vived_models::Server,
        }
        serde_json::from_str::<ServerGetResponse>(raw).map(|r| r.server)
    }
}