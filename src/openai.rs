use reqwest::blocking::Client;
use serde_json::json;
use std::error::Error;

use crate::models::{ChatResponse, Message, Role};

const COMPLETION_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const GPT_4_TURBO_MODEL: &str = "gpt-4-1106-preview";
const SYSTEM_PROMPT: &str = "Write a commit message in conventional commits style. Send response as raw string, no markdown. Do not include scope, so feat: instead of feat(scope):";

pub fn get_chat_response(user_message: Message, api_key: String) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let system_message = Message {
        content: String::from(SYSTEM_PROMPT),
        role: Role::System,
    };

    let res = client
        .post(COMPLETION_API_URL)
        .bearer_auth(api_key)
        .json(
            &json!({ "model": GPT_4_TURBO_MODEL, "messages": vec![system_message, user_message] }),
        )
        .send()?;

    let chat_response: ChatResponse = res.json()?;

    Ok(chat_response.choices[0].message.content.clone())
}
