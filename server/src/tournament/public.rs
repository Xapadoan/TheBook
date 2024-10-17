use std::{error::Error, fmt::Display};

use crate::auth::AuthAPIError;
use crate::repository::RepositoryError;

use super::manager::TournamentManagerError;

#[derive(Debug)]
pub struct TournamentAPIError {
    message: String,
}

impl TournamentAPIError {
    pub fn new(message: &str) -> Self {
        Self { message: format!("Tournament API Error:\n{message}") }
    }
}

impl Display for TournamentAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for TournamentAPIError {}

impl From<TournamentManagerError> for TournamentAPIError {
    fn from(value: TournamentManagerError) -> Self {
        Self::new(&format!("Tournament Manager Error:\n{value}"))
    }
}

impl From<RepositoryError> for TournamentAPIError {
    fn from(value: RepositoryError) -> Self {
        Self::new(&format!("Repository Error:\n{value}"))
    }
}

impl From<AuthAPIError> for TournamentAPIError {
    fn from(value: AuthAPIError) -> Self {
        Self::new(&format!("Auth API Error:\n{value}"))
    }
}
