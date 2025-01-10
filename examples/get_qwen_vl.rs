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
use clap::Parser;
use std::error::Error;
use std::{env, fs};

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    prompt: String,
    image_path: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    // set api key
    let api_key =
        env::var("DASHSCOPE_API_KEY").expect("Missing DASHSCOPE_API_KEY environment variable");

    // config client
    let config = OpenAIConfig::new()
        .with_api_base("https://dashscope.aliyuncs.com/compatible-mode/v1")
        .with_api_key(api_key);
    let client = Client::with_config(config);

    // read image
    let image_bytes = fs::read(args.image_path)?;
    let base64_image = BASE64.encode(image_bytes);
    let data_url = format!("data:image/jpeg;base64,{}", base64_image);

    // create request
    let request = CreateChatCompletionRequestArgs::default()
        .model("qwen-vl-plus")
        .max_tokens(300_u32)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(vec![
                ChatCompletionRequestMessageContentPartTextArgs::default()
                    .text(args.prompt)
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

    //println!("Request: {}", serde_json::to_string_pretty(&request)?);/ print request

    let response = client.chat().create(request).await?;

    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index,
            choice.message.role,
            choice.message.content.unwrap_or_default()
        );
    }

    Ok(())
}
