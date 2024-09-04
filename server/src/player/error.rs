use std::error::Error;
use std::fmt::Display;

use crate::auth::SessionManagerError;
use crate::repository::RepositoryError;
use crate::shop::ShopManagerError;
use crate::tournament::manager::TournamentManagerError;

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

impl From<SessionManagerError> for PlayerAPIError {
    fn from(value: SessionManagerError) -> Self {
        Self::new(&format!("Session Manager Error:\n{value}"))
    }
}

impl From<TournamentManagerError> for PlayerAPIError {
    fn from(value: TournamentManagerError) -> Self {
        Self::new(&format!("Tournament Manager Error:\n{value}"))
    }
}

impl From<ShopManagerError> for PlayerAPIError {
    fn from(value: ShopManagerError) -> Self {
        Self::new(&format!("Shop Manager Error:\n{value}"))
    }
}
