use crate::model::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AIChatRequest {
    pub message: String,
    pub model: String,
}

pub async fn get_ai_response(
    message: &str,
    model: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    match model {
        "openai" => get_openai_response(message).await,
        "qwen" => get_qwen_response(message).await,
        "glm" => get_glm_response(message).await,
        _ => Ok("不支持的模型类型".to_string()),
    }
}
