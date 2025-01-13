use clap::Parser;
use reqwest;
use serde_json::Value;
use std::error::Error;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let client = reqwest::Client::new();

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

    let response = client
        .post(&url)
        .json(&serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": args.prompt
                }]
            }]
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;

    if let Some(text) = response["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        println!("Response: {}", text);
    }

    Ok(())
}
