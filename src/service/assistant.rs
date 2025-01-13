use crate::model::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AIChatRequest {
    pub message: String,
    pub model: String,
}

impl AIChatRequest {
    pub fn new(message: String, model: String) -> Self {
        Self { message, model }
    }

    pub async fn get_ai_response(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self.model.as_str() {
            "openai" => Ok(OPENAI::new(&self.message).get_openai_response().await?),
            "qwen" => Ok(Qwen::new(&self.message).get_qwen_response().await?),
            "glm" => Ok(GLM::new(&self.message).get_glm_response().await?),
            _ => Ok("不支持的模型类型".to_string()),
        }
    }
}
