use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct GLM {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GLMMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GLMRequest {
    model: String,
    messages: Vec<GLMMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GLMResponse {
    choices: Vec<GLMChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GLMChoice {
    message: GLMMessage,
}

impl GLM {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub async fn get_glm_response(&self) -> Result<String, Box<dyn std::error::Error>> {
        let api_key = std::env::var("ZHIPU_API_KEY").expect("ZHIPU_API_KEY must be set");
        let client = reqwest::Client::new();

        let request = GLMRequest {
            model: "glm-4".to_string(),
            messages: vec![
                GLMMessage {
                    role: "system".to_string(),
                    content: "你是一个有帮助的AI助手".to_string(),
                },
                GLMMessage {
                    role: "user".to_string(),
                    content: self.message.clone(),
                },
            ],
        };

        let response = client
            .post("https://open.bigmodel.cn/api/paas/v4/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request)
            .send()
            .await?
            .json::<GLMResponse>()
            .await?;

        Ok(response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_else(|| "No response from GLM".to_string()))
    }
}
