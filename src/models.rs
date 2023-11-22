use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub openai_api_key: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub role: Role,
}

#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Serialize, Deserialize)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}
