use crate::api::websocket::message::{ClientMessage, ServerMessage};
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::{IntoResponse, Response};
use futures_util::{SinkExt, StreamExt};

pub async fn websocket_upgrade(upgrade: WebSocketUpgrade) -> Response {
  upgrade.on_upgrade(handle_socket).into_response()
}

async fn handle_socket(socket: WebSocket) {
  let (mut sender, mut receiver) = socket.split();
  while let Some(Ok(Message::Text(text))) = receiver.next().await {
    let Ok(message) = serde_json::from_str::<ClientMessage>(&text) else {
      continue;
    };
    let response = ServerMessage { body: message.body };
    let Ok(response) = serde_json::to_string(&response) else {
      continue;
    };
    if sender.send(Message::Text(response.into())).await.is_err() {
      break;
    }
  }
}
