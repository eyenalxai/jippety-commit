use std::error::Error;

use crate::config::get_api_key;
use crate::diff::get_diff;
use crate::models::{Message, Role};
use crate::openai::get_chat_response;

mod config;
mod diff;
mod models;
mod openai;

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
