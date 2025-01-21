use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::services::ServeDir;

use super::{
    chat::handle_ai_chat,
    login::{handle_login, serve_chat, serve_login},
    middleware::auth_middleware,
    upload::handle_upload,
    ws::{ws_handler, AppState},
};

pub fn create_router(app_state: Arc<AppState>, upload_dir: String) -> Router {
    Router::new()
        // 不需要认证的路由
        .route("/", get(serve_login))
        .route("/login", post(handle_login))
        .route("/chat", get(serve_chat))
        // WebSocket 路由
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
        .nest_service("/uploads", ServeDir::new(upload_dir))
        .with_state(app_state)
}
