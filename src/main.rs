use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Multipart, Query, State,
    },
    http::{Request, Response, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use futures::{sink::SinkExt, stream::StreamExt};
use netchat::{create_token, verify_token, LoginRequest};
use netchat::{get_ai_response, AIChatRequest};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    #[serde(rename = "userId")]
    user_id: String,
    message: String,
}

#[derive(Serialize)]
struct UploadResponse {
    url: String,
}

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<String>,
}

async fn auth_middleware(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    verify_token(auth.token()).map_err(|_| StatusCode::UNAUTHORIZED)?;
    Ok(next.run(request).await)
}

#[derive(Deserialize)]
struct WsQuery {
    token: String,
    username: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    netchat::set_logger();
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

    let app = Router::new()
        // 不需要认证的路由
        .route("/", get(serve_login))
        .route("/login", post(handle_login))
        .route("/chat", get(serve_chat))
        // WebSocket 路由单独处理
        .route("/api/ws", get(ws_handler))
        // 需要认证的路由组
        .nest(
            "/api",
            Router::new()
                .route("/ai_chat", post(handle_ai_chat))
                .route("/upload", post(handle_upload))
                .layer(middleware::from_fn(auth_middleware)),
        )
        // 静态文件服务
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/uploads", ServeDir::new(&upload_dir))
        .with_state(app_state);

    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
    tracing::info!("Start NetChat Service!");
    tracing::info!("Service start in http://{}:{}", host, port);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
async fn serve_login() -> impl IntoResponse {
    Html(include_str!("../static/login.html"))
}

async fn serve_chat() -> impl IntoResponse {
    Html(include_str!("../static/chat.html"))
}

async fn handle_login(
    Json(login): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // 这里应该查询数据库验证用户名和密码
    // 现在简单实现，只要用户名不为空就允许登录
    if !login.username.is_empty() {
        let token = create_token(&login.username).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(serde_json::json!({
            "token": token,
            "username": login.username
        })))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn handle_ai_chat(
    Json(request): Json<AIChatRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match get_ai_response(&request.message).await {
        Ok(response) => Ok(Json(serde_json::json!({ "response": response }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Query(query): Query<WsQuery>,
) -> impl IntoResponse {
    // 验证 token
    if let Err(_) = verify_token(&query.token) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    // 将用户名传递给 handle_socket
    ws.on_upgrade(move |socket| handle_socket(socket, state, query.username))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, username: String) {
    tracing::info!("用户 {} 已连接", username);

    // 创建一个广播接收器
    let mut rx = state.tx.subscribe();
    let (mut sender, mut receiver) = socket.split();

    // 发送任务：接收广播消息并发送给客户端
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // 接收任务：接收客户端消息并广播
    let tx = state.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            tracing::info!("收到来自 {} 的消息: {}", username, text);
            if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&text) {
                if let Ok(json) = serde_json::to_string(&chat_msg) {
                    tracing::info!("广播消息: {}", json);
                    // 使用 tx 广播消息给所有连接的客户端
                    let _ = tx.send(json);
                }
            }
        }
        tracing::info!("用户 {} 已断开连接", username);
    });

    // 等待任意一个任务完成
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

async fn handle_upload(mut multipart: Multipart) -> impl IntoResponse {
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "uploads".to_string());
    if !Path::new(&upload_dir).exists() {
        fs::create_dir(&upload_dir).await.unwrap();
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name().unwrap() == "file" {
            let filename = field.file_name().unwrap().to_string();
            let filepath = format!("{}/{}", upload_dir, filename);

            let data = field.bytes().await.unwrap();
            fs::write(&filepath, data).await.unwrap();

            let url = format!("/uploads/{}", filename);
            return Json(UploadResponse { url });
        }
    }

    Json(UploadResponse {
        url: "error".to_string(),
    })
}
