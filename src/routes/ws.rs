use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    #[serde(rename = "userId")]
    user_id: String,
    message: String,
}

#[derive(Deserialize)]
pub struct WsQuery {
    token: String,
    username: String,
}

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<String>,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Query(query): Query<WsQuery>,
) -> impl IntoResponse {
    if let Err(_) = crate::verify_token(&query.token) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    ws.on_upgrade(move |socket| handle_socket(socket, state, query.username))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, username: String) {
    tracing::info!("用户 {} 已连接", username);

    let mut rx = state.tx.subscribe();
    let (mut sender, mut receiver) = socket.split();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            tracing::info!("收到来自 {} 的消息: {}", username, text);
            if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&text) {
                if let Ok(json) = serde_json::to_string(&chat_msg) {
                    tracing::info!("广播消息: {}", json);
                    let _ = tx.send(json);
                }
            }
        }
        tracing::info!("用户 {} 已断开连接", username);
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
