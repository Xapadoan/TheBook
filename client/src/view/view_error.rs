use std::error::Error;
use std::fmt::Display;
use std::io;

use shared::player::PlayerBuildError;

use crate::auth::SessionError;
use crate::fetcher::ApiFetcherError;
use crate::prompt::PromptError;

#[derive(Debug)]
pub struct ViewError {
    message: String,
}

impl ViewError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for ViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ViewError {}

impl From<SessionError> for ViewError {
    fn from(value: SessionError) -> Self {
        Self::new(format!("Session Error:\n{value}"))
    }
}

impl From<PlayerBuildError> for ViewError {
    fn from(value: PlayerBuildError) -> Self {
        Self::new(format!("Player Build Error:\n{value}"))
    }
}

impl From<PromptError> for ViewError {
    fn from(value: PromptError) -> Self {
        Self::new(format!("Prompt Error:\n{value}"))
    }
}

impl From<ureq::Error> for ViewError {
    fn from(value: ureq::Error) -> Self {
        Self::new(format!("Raw Request Error:\n{value}"))
    }
}

impl From<ApiFetcherError> for ViewError {
    fn from(value: ApiFetcherError) -> Self {
        Self::new(format!("ureq Error:\n{value}"))
    }
}

impl From<io::Error> for ViewError {
    fn from(value: io::Error) -> Self {
        Self::new(format!("io Error:\n{value}"))
    }
}

impl From<dotenv::Error> for ViewError {
    fn from(value: dotenv::Error) -> Self {
        Self::new(format!("dotenv Error:\n{value}"))
    }
}
