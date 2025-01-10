use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent, CreateChatCompletionRequest,
    },
    Client,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AIChatRequest {
    pub message: String,
}

pub async fn get_ai_response(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: ChatCompletionRequestUserMessageContent::Text(message.to_string()),
                name: None,
            },
        )],
        ..Default::default()
    };

    let response = client.chat().create(request).await?;

    Ok(response.choices[0]
        .message
        .content
        .clone()
        .unwrap_or_default())
}
