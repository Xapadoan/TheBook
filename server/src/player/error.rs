use std::error::Error;
use std::fmt::Display;

use crate::repository::RepositoryError;

#[derive(Debug)]
pub struct PlayerAPIError {
    message: String,
}

impl PlayerAPIError {
    pub fn new(message: &str) -> Self {
        Self { message: format!("Player API Error:\n{message}") }
    }
}

impl Display for PlayerAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PlayerAPIError {}

impl From<RepositoryError> for PlayerAPIError {
    fn from(value: RepositoryError) -> Self {
        Self::new(&format!("Repository Error:\n{value}"))
    }
}
