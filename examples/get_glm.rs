use clap::Parser;
use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    prompt: String,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    index: i32,
    message: ResponseMessage,
}

#[derive(Deserialize, Debug)]
struct ResponseMessage {
    role: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let api_key = std::env::var("ZHIPU_API_KEY").expect("ZHIPU_API_KEY must be set");

    let client = reqwest::Client::new();

    let request = ChatRequest {
        model: "glm-4-plus".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "你是一个ai助手".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: args.prompt,
            },
        ],
    };

    let response = client
        .post("https://open.bigmodel.cn/api/paas/v4/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?
        .json::<ChatResponse>()
        .await?;

    for choice in response.choices {
        println!(
            "{}: Role: {} Content: {}",
            choice.index, choice.message.role, choice.message.content
        );
    }

    Ok(())
}
