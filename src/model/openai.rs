use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent, CreateChatCompletionRequest,
    },
    Client,
};

#[derive(Debug)]
pub struct OPENAI {
    message: String,
}
impl OPENAI {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
    pub async fn get_openai_response(&self) -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::new();
        let request = CreateChatCompletionRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessage {
                    content: ChatCompletionRequestUserMessageContent::Text(self.message.clone()),
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
}
