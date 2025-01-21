use crate::{create_token, LoginRequest};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Json},
};

pub async fn serve_login() -> impl IntoResponse {
    Html(include_str!("../../static/login.html"))
}

pub async fn serve_chat() -> impl IntoResponse {
    Html(include_str!("../../static/chat.html"))
}

pub async fn handle_login(
    Json(login): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
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
