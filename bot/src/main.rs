use vived::{endpoints, ApiClient, connect_to_websocket};

const TOKEN: &str = include_str!("../TOKEN");
const TEST_CHANNEL_ID: &str = "c1271f4d-27ef-42b6-81f8-bc4e1b0947f4";

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut events = connect_to_websocket(TOKEN, 10).await.unwrap();

    while let Ok(event) = events.recv().await {
        dbg!(event);
    }
}
