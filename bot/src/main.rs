use vived::{endpoints::MessageCreate, Client};

const TOKEN: &str = include_str!("../TOKEN");
const TEST_CHANNEL_ID: &str = "c1271f4d-27ef-42b6-81f8-bc4e1b0947f4";

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = Client::new(TOKEN);

    // Send startup embed message
    let msg = client
        .make_request(
            MessageCreate::new_with_embed(
                TEST_CHANNEL_ID,
                vived::Embed::new().description("test test <@dVBx8aZd>"),
            )
        )
        .await
        .unwrap();
    dbg!(msg);
}
