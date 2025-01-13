use reqwest;
use serde::{Deserialize, Serialize};

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

pub async fn get_glm_response(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = std::env::var("ZHIPU_API_KEY").expect("ZHIPU_API_KEY must be set");
    let client = reqwest::Client::new();

    let request = GLMRequest {
        model: "glm-4-plus".to_string(),
        messages: vec![
            GLMMessage {
                role: "system".to_string(),
                content: "你是一个有帮助的AI助手".to_string(),
            },
            GLMMessage {
                role: "user".to_string(),
                content: message.to_string(),
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

    // 获取第一个回复的内容
    Ok(response
        .choices
        .first()
        .map(|choice| choice.message.content.clone())
        .unwrap_or_else(|| "No response from GLM".to_string()))
}
