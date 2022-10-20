use vived::{endpoints::MessageCreate, ApiClient};

const TOKEN: &str = include_str!("../TOKEN");
const TEST_CHANNEL_ID: &str = "c1271f4d-27ef-42b6-81f8-bc4e1b0947f4";

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = ApiClient::new(TOKEN);

    let mut count = 0;
    for _ in 0..10 {
        count += 1;
        client
            .make_request(MessageCreate::new(TEST_CHANNEL_ID.into()).with_content(&count.to_string()))
            .await
            .unwrap();
    }

}
