use std::{error, fmt, io, num};

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

impl fmt::Display for PromptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for PromptError {}

impl From<num::ParseIntError> for PromptError {
    fn from(value: num::ParseIntError) -> Self {
        Self::new(&format!("Parse Int Error: \n{value}"))
    }
}
