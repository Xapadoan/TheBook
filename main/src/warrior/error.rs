use std::error::Error;
use std::fmt::Display;

use crate::repository::RepositoryError;

#[derive(Debug)]
pub struct WarriorAPIError {
    message: String,
}

impl WarriorAPIError {
    pub fn new(message: &str) -> Self {
        Self { message: format!("Warrior API Error:\n{message}") }
    }
}

impl Display for WarriorAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WarriorAPIError {}

impl From<RepositoryError> for WarriorAPIError {
    fn from(value: RepositoryError) -> Self {
        Self::new(&format!("Repository Error:\n{value}"))
    }
}
