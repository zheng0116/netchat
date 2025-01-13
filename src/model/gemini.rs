use reqwest;
use serde_json::{json, Value};

pub struct Gemini {
    message: String,
}

impl Gemini {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub async fn get_gemini_response(&self) -> Result<String, Box<dyn std::error::Error>> {
        let api_key = std::env::var("GEMINI_API_KEY")?;
        let client = reqwest::Client::new();

        let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

        let response = client
            .post(&url)
            .json(&json!({
                "contents": [
                    {
                        "parts": [
                            {
                                "text": self.message
                            }
                        ]
                    }
                ]
            }))
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(response["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or_default()
            .to_string())
    }
}
