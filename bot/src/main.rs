use vived::{endpoints::MessageCreate, Client};

const TOKEN: &str = include_str!("../TOKEN");
const TEST_CHANNEL_ID: &str = "c1271f4d-27ef-42b6-81f8-bc4e1b0947f4";

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = Client::new(TOKEN);

    // Send startup message to server
    let resulting_message = client
        .make_request(
            MessageCreate::new(TEST_CHANNEL_ID.to_owned().into())
                .with_content("I am online".into()),
        )
        .await
        .unwrap();
    
    dbg!(resulting_message);
}
