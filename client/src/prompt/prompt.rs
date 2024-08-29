use std::io;

use super::PromptError;

pub fn prompt_bool(message: &str) -> Result<bool, PromptError> {
    let user_response = prompt(format!("{message} (Y / N)").as_str())?;
    Ok(user_response.to_lowercase() == "y")
}

pub fn prompt(message: &str) -> Result<String, PromptError> {
    println!("{message}");
    let mut user_response = String::new();
    io::stdin().read_line(&mut user_response)?;
    Ok(String::from(user_response.trim()))
}
