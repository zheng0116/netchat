use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageContentPartTextArgs,
        ChatCompletionRequestSystemMessage, ChatCompletionRequestSystemMessageContent,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
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
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
                content: ChatCompletionRequestSystemMessageContent::Text(
                    "你是一个ai助手".to_string(),
                ),
                name: None,
            }),
            ChatCompletionRequestUserMessageArgs::default()
                .content(vec![
                    ChatCompletionRequestMessageContentPartTextArgs::default()
                        .text(args.prompt)
                        .build()?
                        .into(),
                ])
                .build()?
                .into(),
        ])
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
