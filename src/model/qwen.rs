use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent, CreateChatCompletionRequest,
    },
    Client,
};
#[derive(Debug)]
pub struct Qwen {
    message: String,
}
impl Qwen {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
    pub async fn get_qwen_response(&self) -> Result<String, Box<dyn std::error::Error>> {
        let api_key = std::env::var("DASHSCOPE_API_KEY")?;
        let config = OpenAIConfig::new()
            .with_api_base("https://dashscope.aliyuncs.com/compatible-mode/v1")
            .with_api_key(api_key);
        let client = Client::with_config(config);

        let request = CreateChatCompletionRequest {
            model: "qwen-turbo".to_string(),
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
