use crate::AIChatRequest;
use axum::{http::StatusCode, response::Json};

pub async fn handle_ai_chat(
    Json(request): Json<AIChatRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    tracing::info!("Received AI chat request: {:?}", request);
    match request.get_ai_response().await {
        Ok(response) => {
            tracing::info!("AI response success: {}", response);
            Ok(Json(serde_json::json!({ "response": response })))
        }
        Err(e) => {
            tracing::info!("AI response error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
