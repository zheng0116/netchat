use crate::model::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AIChatRequest {
    pub message: String,
    pub model: String,
    #[serde(default)]
    pub image_path: Option<String>,
}

impl AIChatRequest {
    pub fn new(message: String, model: String) -> Self {
        Self {
            message,
            model,
            image_path: None,
        }
    }

    pub async fn get_ai_response(&self) -> Result<String, Box<dyn std::error::Error>> {
        tracing::info!(
            "Processing request - Model: {}, Image path: {:?}",
            self.model,
            self.image_path,
        );

        match self.model.as_str() {
            "openai" => Ok(OPENAI::new(&self.message).get_openai_response().await?),
            "qwen" => Ok(Qwen::new(&self.message).get_qwen_response().await?),
            "glm" => Ok(GLM::new(&self.message).get_glm_response().await?),
            "gemini" => Ok(Gemini::new(&self.message).get_gemini_response().await?),
            "qwen_vl" => {
                tracing::info!("Handling qwen_vl request");
                if let Some(image_path) = &self.image_path {
                    tracing::info!("Using image path: {}", image_path);
                    Ok(QwenVl::new(self.message.clone(), image_path.clone())
                        .get_qwen_vl_response()
                        .await?)
                } else {
                    tracing::info!("No image path provided for qwen_vl");
                    Ok("需要上传图片才能使用 Qwen VL 模型".to_string())
                }
            }
            _ => {
                tracing::info!("Unsupported model type: {}", self.model);
                Ok("不支持的模型类型".to_string())
            }
        }
    }
}
