//! Websocket client

use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;

/// Where to connect to.
const WEBSOCKET_ENDPOINT: &str = "wss://www.guilded.gg/websocket/v1";
// const WEBSOCKET_ENDPOINT: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

use tokio_tungstenite::tungstenite::{self, client::IntoClientRequest};

/// Websocket stream
type WebStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

/// Create the websocket connection.
async fn create_connection(
    request: impl tungstenite::client::IntoClientRequest + Unpin,
) -> Result<WebStream, tungstenite::Error> {
    let (connection, _response) = tokio_tungstenite::connect_async(request).await?;
    Ok(connection)
}

/// Connect to the websocket with the provided token.
///
/// `event_capacity` is the capacity of the event queue.
/// see [`tokio::sync::broadcast::channel`] for more info.
///
/// # Errors
/// If the token is an invalid header value or the connection fails.
pub async fn connect_to_websocket(
    token: &str,
    event_capacity: usize,
) -> Result<broadcast::Receiver<crate::events::GuildedEvent>, tungstenite::Error> {
    let user_agent = format!(
        "library: vived, version: {}, rustc version: {}",
        version::version!(),
        rustc_version_runtime::version()
    );

    let mut request = WEBSOCKET_ENDPOINT.into_client_request()?;
    let headers = request.headers_mut();
    headers.insert("Authorization", format!("Bearer {token}").parse()?);
    headers.insert("User-Agent", user_agent.parse()?);

    log::debug!("connecting to websocket");
    let connection = create_connection(request).await?;
    let (tx, rx) = tokio::sync::broadcast::channel(event_capacity);

    tokio::spawn(event_loop(connection, tx));

    Ok(rx)
}

/// The event loop for the websocket.
async fn event_loop(connection: WebStream, tx: broadcast::Sender<crate::events::GuildedEvent>) {
    let (mut write, mut read) = connection.split();

    while let Some(message) = read.next().await {
        let message = match message {
            Ok(message) => message,
            Err(e) => {
                log::error!("error reading from websocket: {}", e);
                continue;
            }
        };

        let message = match message {
            tungstenite::Message::Text(text) => text,
            tungstenite::Message::Binary(binary) => match String::from_utf8(binary) {
                Ok(text) => text,
                Err(e) => {
                    log::error!("error converting binary message to text: {e}");
                    continue;
                }
            },
            tungstenite::Message::Ping(ping) => {
                if let Err(e) = write.send(tungstenite::Message::Pong(ping)).await {
                    log::error!("error sending pong: {e}");
                }
                continue;
            }
            _ => {
                log::error!("received non-text message from websocket");
                continue;
            }
        };

        let event: crate::events::GuildedEvent = match serde_json::from_str(&message) {
            Ok(event) => event,
            Err(e) => {
                log::error!("error deserializing event: {e}");
                log::debug!("raw event: {message}");
                continue;
            }
        };

        log::debug!("received event: {:?}", event);

        if let Err(e) = tx.send(event) {
            log::error!("error sending event: {}", e);
        }
    }
}
