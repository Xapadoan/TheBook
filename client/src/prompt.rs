use std::error::Error;
use std::fmt::Display;
use std::io;

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

#[derive(Debug)]
pub struct PromptError {
    message: String,
}

impl PromptError {
    fn new(message: &str) -> Self {
        Self { message: format!("PromptError:\n{message}")}
    }
}

impl From<io::Error> for PromptError {
    fn from(value: io::Error) -> Self {
        Self::new(&format!("io::Error:\n{value}"))
    }
}

impl Display for PromptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PromptError {}
