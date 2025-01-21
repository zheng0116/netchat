use netchat::set_logger;
use netchat::AIChatRequest;
use netchat::{create_token, verify_token, LoginRequest};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::broadcast;

mod routes;
use routes::{router::create_router, ws::AppState};

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    #[serde(rename = "userId")]
    user_id: String,
    message: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    set_logger();
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "uploads".to_string());
    let _max_file_size: u64 = std::env::var("MAX_FILE_SIZE")
        .unwrap_or_else(|_| "10485760".to_string())
        .parse()
        .unwrap_or(10485760);

    if !Path::new(&upload_dir).exists() {
        fs::create_dir(&upload_dir).await.unwrap();
    }

    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState { tx });

    let app = create_router(app_state, upload_dir);

    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
    tracing::info!("Start NetChat Service!");
    tracing::info!("Service start in http://{}:{}", host, port);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
