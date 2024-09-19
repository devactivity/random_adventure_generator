use color_eyre::eyre::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    temperature: f32,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

pub async fn generate_adventure(genre: &str, difficulty: &str, length: &str) -> Result<String> {
    let client = Client::new();

    let request = ChatCompletionRequest {
        model: "gpt-4".to_string(),
        temperature: 0.7,
        messages: vec![Message {
            role: "user".to_string(),
            content: format!(
                "Gennerate an adventure with the following settings:\nGenre: {}\nDifficulty: {}\nLength: {}\n\nProvide the following details:\n1. Location\n2. Characters (comma-separated)\n3. Objective\n4. Challenges (comma-separated)",
                genre, difficulty, length
            )
        }]
    };

    let response: ChatCompletionResponse = client
        .post("http://localhost:8080/v1/chat/completions")
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    Ok(response.choices[0].message.content.clone())
}
