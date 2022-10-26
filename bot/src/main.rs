use vived::{connect_to_websocket, endpoints, events::GuildedEvent, ApiClient};

const TOKEN: &str = include_str!("../TOKEN");
const TEST_CHANNEL_ID: &str = "c1271f4d-27ef-42b6-81f8-bc4e1b0947f4";
// const TEST_CHANNEL_ID: &str = "054d5589-75d6-43f2-a4cd-f20e9a4a5954";

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = ApiClient::new(TOKEN).unwrap();
    let channel = client
        .make_request(endpoints::GetChannel::new(TEST_CHANNEL_ID))
        .await
        .unwrap();
    dbg!(channel);

    // let mut events = connect_to_websocket(TOKEN, 10).await.unwrap();

    // while let Ok(event) = events.recv().await {
    //     if let GuildedEvent::ChatMessageCreated { server_id, message } = event {
    //         let server = client
    //             .make_request(endpoints::GetServer::new(server_id))
    //             .await
    //             .unwrap();

    //         println!("{}: {:?}", server.name, message.content);
    //     }
    // }
}
