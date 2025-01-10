use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessageContentPartTextArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    prompt: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let api_key = std::env::var("DASHSCOPE_API_KEY").expect("DASHSCOPE_API_KEY must be set");
    let config = OpenAIConfig::new()
        .with_api_base("https://dashscope.aliyuncs.com/compatible-mode/v1")
        .with_api_key(api_key);
    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .model("qwen-plus")
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(vec![
                ChatCompletionRequestMessageContentPartTextArgs::default()
                    .text(args.prompt)
                    .build()?
                    .into(),
            ])
            .build()?
            .into()])
        .build()?;

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
