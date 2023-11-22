use std::error::Error;
use std::fs::read_to_string;
use std::process::Command;

use dirs::home_dir;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    openai_api_key: String,
}

const CONFIG_LOCATION: &str = ".config/jippety-kohmmit/config.toml";

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

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = match get_api_key() {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };
    println!("{:?}", api_key);
    Ok(())
}
