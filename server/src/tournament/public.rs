use std::{error::Error, fmt::Display, path::PathBuf};

use shared::tournament::contestant::TournamentContestant;
use shared::tournament::Tournament;
use shared::warrior::Warrior;
use uuid::Uuid;

use crate::auth::AuthAPIError;
use crate::repository::{FileRepository, Repository, RepositoryError};

use super::manager::{TournamentManager, TournamentManagerError};

pub fn playable_tournament() -> Result<Tournament, TournamentAPIError> {
    let manager = TournamentManager::build()?;
    let tournament = manager.get_playable_tournament()?;
    Ok(tournament)
}

pub fn remove_contestant(warrior_uuid: &Uuid) -> Result<(), TournamentAPIError> {
    eprintln!("Removing contestant {warrior_uuid}");
    let repo = FileRepository::build(PathBuf::from("saves/warriors"))?;
    let mut warrior: Warrior = repo.get_by_uuid(warrior_uuid)?;
    eprintln!("GET OK");
    warrior.set_current_tournament(None);
    repo.update(warrior_uuid, &warrior)?;
    eprintln!("UPDATE OK");
    Ok(())
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
