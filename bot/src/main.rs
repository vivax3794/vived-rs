use vived::{endpoints, Client};

const TOKEN: &str = include_str!("../TOKEN");
const TEST_CHANNEL_ID: &str = "c1271f4d-27ef-42b6-81f8-bc4e1b0947f4";

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = Client::new(TOKEN);

    // Send message and edit it 2 seconds later
    let message = client
        .make_request(endpoints::MessageCreate::new_with_content(
            TEST_CHANNEL_ID,
            "Hello World!",
        ))
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let message = client
        .make_request(
            endpoints::MessageEdit::new(TEST_CHANNEL_ID, message).content("Hello Guilded!"),
        )
        .await
        .unwrap();
    
    // wait 3 more seconds and delete the message
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    client
        .make_request(endpoints::MessageDelete::new(TEST_CHANNEL_ID, message))
        .await
        .unwrap();
}
