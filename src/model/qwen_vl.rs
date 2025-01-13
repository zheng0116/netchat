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

pub async fn get_qwen_vl_response(
    prompt: String,
    image_path: String,
) -> Result<String, Box<dyn Error>> {
    let api_key =
        env::var("DASHSCOPE_API_KEY").expect("Missing DASHSCOPE_API_KEY environment variable");

    // config client
    let config = OpenAIConfig::new()
        .with_api_base("https://dashscope.aliyuncs.com/compatible-mode/v1")
        .with_api_key(api_key);
    let client = Client::with_config(config);

    // read image
    let image_bytes = fs::read(image_path)?;
    let base64_image = BASE64.encode(image_bytes);
    let data_url = format!("data:image/jpeg;base64,{}", base64_image);

    // create request
    let request = CreateChatCompletionRequestArgs::default()
        .model("qwen-vl-plus")
        .max_tokens(300_u32)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(vec![
                ChatCompletionRequestMessageContentPartTextArgs::default()
                    .text(prompt)
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

    // 获取第一个回复的内容并返回
    Ok(response
        .choices
        .first()
        .and_then(|choice| choice.message.content.clone())
        .unwrap_or_else(|| "No response from Qwen-VL".to_string()))
}
