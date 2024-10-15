use std::{error::Error, fmt::Display};

use shared::tournament::Tournament;

use crate::auth::AuthAPIError;
use crate::repository::RepositoryError;

use super::manager::{TournamentManager, TournamentManagerError};

pub fn playable_tournament() -> Result<Tournament, TournamentAPIError> {
    let manager = TournamentManager::build()?;
    let tournament = manager.get_playable_tournament()?;
    Ok(tournament)
}

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
