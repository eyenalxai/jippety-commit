use std::error::Error;
use std::fs::read_to_string;

use dirs::home_dir;

use crate::models::Config;

const CONFIG_LOCATION: &str = ".config/jippety-kohmmit/config.toml";

fn get_config() -> Result<Config, Box<dyn Error>> {
    let home_folder = home_dir().ok_or("Home directory not found")?;
    let config_path = home_folder.join(CONFIG_LOCATION);
    let contents =
        read_to_string(config_path).map_err(|e| format!("Failed to read config\n{}", e))?;
    Ok(toml::from_str(&contents)?)
}

pub fn get_api_key() -> Result<String, Box<dyn Error>> {
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
