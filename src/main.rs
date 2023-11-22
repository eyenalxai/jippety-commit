use std::error::Error;
use std::fs::read_to_string;
use std::process::Command;

use dirs::home_dir;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use serde_json::json;

#[derive(Deserialize, Debug)]
struct Config {
    openai_api_key: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Role {
    System,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
    role: Role,
}

#[derive(Serialize, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Serialize, Deserialize)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

const CONFIG_LOCATION: &str = ".config/jippety-kohmmit/config.toml";
const COMPLETION_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const GPT_4_TURBO_MODEL: &str = "gpt-4-1106-preview";
const SYSTEM_PROMPT: &str = "Write a commit message in conventional commits style. Send response as raw string, no markdown. Do not include scope, so feat: instead of feat(scope):";
fn get_config() -> Result<Config, Box<dyn Error>> {
    let home_folder = home_dir().ok_or("Home directory not found")?;
    let config_path = home_folder.join(CONFIG_LOCATION);
    let contents =
        read_to_string(config_path).map_err(|e| format!("Failed to read config\n{}", e))?;
    Ok(toml::from_str(&contents)?)
}

fn get_api_key() -> Result<String, Box<dyn Error>> {
    let config = get_config()?;
    let api_key = config.openai_api_key;

    if !api_key.starts_with("sk-") {
        return Err("OpenAI API key must start with sk-".into());
    }

    if api_key.len() < 32 {
        return Err("OpenAI API key is too short".into());
    }

    Ok(api_key)
}

fn get_diff() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(["diff", "--staged", "--minimal"])
        .output()?;

    if !output.status.success() {
        return Err("Failed to execute git diff".into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

fn get_chat_response(user_message: Message, api_key: String) -> Result<String, Box<dyn Error>> {
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

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = match get_api_key() {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    let diff = match get_diff() {
        Ok(diff) => diff,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    let user_message = Message {
        content: diff,
        role: Role::User,
    };

    let chat_response = match get_chat_response(user_message, api_key) {
        Ok(chat_response) => chat_response,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    println!("{}", chat_response);
    Ok(())
}
