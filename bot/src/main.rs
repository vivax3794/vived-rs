use vived::{endpoints, Client};

const TOKEN: &str = include_str!("../TOKEN");
const TEST_CHANNEL_ID: &str = "c1271f4d-27ef-42b6-81f8-bc4e1b0947f4";

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = Client::new(TOKEN).unwrap();

    // Send a little hello embed
    client
        .make_request(endpoints::MessageCreate::new_with_embed(
            TEST_CHANNEL_ID,
            vived::Embed::new()
                .title("Hello, world!")
                .description("This is a test embed.")
                .color(0x00FF00),
        ))
        .await
        .unwrap();
    
    
}
