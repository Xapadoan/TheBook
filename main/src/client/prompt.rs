use std::{error::Error, fmt::Display, io};

pub fn prompt_bool(message: &str) -> Result<bool, PromptError> {
    println!("{message} (Y / N)");
    let mut user_response = String::new();
    io::stdin().read_line(&mut user_response)?;
    if user_response.trim().to_lowercase() == "y" {
        Ok(true)
    } else {
        Ok(false)
    }
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
