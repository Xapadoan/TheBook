use std::{error::Error, fmt::Display};

use shared::player::PlayerBuildError;

use crate::repository::RepositoryError;

#[derive(Debug)]
pub struct AuthAPIError {
    message: String,
}

impl AuthAPIError {
    fn new(message: String) -> Self {
        Self { message: format!("Session Error\n{message}") }
    }
}

impl Display for AuthAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AuthAPIError {}

impl From<RepositoryError> for AuthAPIError {
    fn from(value: RepositoryError) -> Self {
        Self::new(format!("Repository Error:\n{value}"))
    }
}

impl From<AuthAPIError> for PlayerBuildError {
    fn from(value: AuthAPIError) -> Self {
        eprintln!("[WARN] Should remove impl From<AuthAPIError> for PlayerBuildError");
        Self::new(format!("Auth API Error:\n{value}"))
    }
}
