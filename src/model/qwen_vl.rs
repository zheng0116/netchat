use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessageContentPartImageArgs,
        ChatCompletionRequestMessageContentPartTextArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, ImageDetail, ImageUrlArgs,
    },
    Client,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use std::error::Error;
use std::{env, fs};
#[derive(Debug)]
pub struct QwenVl {
    prompt: String,
    image_path: String,
}

impl QwenVl {
    pub fn new(prompt: String, image_path: String) -> Self {
        Self { prompt, image_path }
    }

    pub async fn get_qwen_vl_response(&self) -> Result<String, Box<dyn Error>> {
        let api_key =
            env::var("DASHSCOPE_API_KEY").expect("Missing DASHSCOPE_API_KEY environment variable");

        // config client
        let config = OpenAIConfig::new()
            .with_api_base("https://dashscope.aliyuncs.com/compatible-mode/v1")
            .with_api_key(api_key);
        let client = Client::with_config(config);

        let image_path = if self.image_path.starts_with('/') {
            format!(".{}", self.image_path)
        } else {
            self.image_path.clone()
        };

        tracing::info!("Trying to read image from path: {}", image_path);

        let image_bytes = fs::read(&image_path)?;
        let base64_image = BASE64.encode(image_bytes);
        let data_url = format!("data:image/jpeg;base64,{}", base64_image);

        let request = CreateChatCompletionRequestArgs::default()
            .model("qwen-vl-plus")
            .max_tokens(300_u32)
            .messages([ChatCompletionRequestUserMessageArgs::default()
                .content(vec![
                    ChatCompletionRequestMessageContentPartTextArgs::default()
                        .text(self.prompt.clone())
                        .build()?
                        .into(),
                    ChatCompletionRequestMessageContentPartImageArgs::default()
                        .image_url(
                            ImageUrlArgs::default()
                                .url(&data_url)
                                .detail(ImageDetail::High)
                                .build()?,
                        )
                        .build()?
                        .into(),
                ])
                .build()?
                .into()])
            .build()?;

        let response = client.chat().create(request).await?;

        Ok(response
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .unwrap_or_else(|| "No response from Qwen-VL".to_string()))
    }
}
