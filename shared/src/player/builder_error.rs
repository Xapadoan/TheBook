use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct PlayerBuildError {
    message: String,
}

impl PlayerBuildError {
    pub fn new(message: String) -> Self {
        Self {
            message: format!("PlayerBuildError:\n{message}")
        }
    }
}

impl Display for PlayerBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PlayerBuildError {}
