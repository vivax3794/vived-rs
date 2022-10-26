use vived::{connect_to_websocket, endpoints, ApiClient};

const TOKEN: &str = include_str!("../TOKEN");
const TEST_CHANNEL_ID: &str = "c1271f4d-27ef-42b6-81f8-bc4e1b0947f4";

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = ApiClient::new(TOKEN).unwrap();
    let server = client
        .make_request(endpoints::GetServer::new("gRGbzdWj"))
        .await
        .unwrap();
    dbg!(&server);
    dbg!(server.url());

    let mut events = connect_to_websocket(TOKEN, 10).await.unwrap();

    while let Ok(event) = events.recv().await {
        dbg!(event);
    }
}
