use std::error::Error;
use std::fmt::Display;

use shared::player::PlayerBuildError;

use crate::client::player_logger::PlayerLoggerError;
use crate::client::prompt::PromptError;
use crate::client::select_warrior::SelectWarriorError;
use crate::repository::RepositoryError;
use crate::tournament::manager::TournamentManagerError;
use crate::tournament::replay::manager::ReplayManagerError;

#[derive(Debug)]
pub struct ViewError {
    message: String,
}

impl ViewError {
    fn new(message: String) -> Self {
        Self { message: format!("Tournament Replay Build Error\n{message}") }
    }
}

impl Display for ViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ViewError {}

impl From<RepositoryError> for ViewError {
    fn from(value: RepositoryError) -> Self {
        Self::new(format!("Repository Error:\n{value}"))
    }
}

impl From<PlayerLoggerError> for ViewError {
    fn from(value: PlayerLoggerError) -> Self {
        Self::new(format!("Repository Error:\n{value}"))
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

impl From<ReplayManagerError> for ViewError {
    fn from(value: ReplayManagerError) -> Self {
        Self::new(format!("Replay Manager Error:\n{value}"))
    }
}

impl From<SelectWarriorError> for ViewError {
    fn from(value: SelectWarriorError) -> Self {
        Self::new(format!("Select Warrior Error:\n{value}"))
    }
}

impl From<TournamentManagerError> for ViewError {
    fn from(value: TournamentManagerError) -> Self {
        Self::new(format!("Tournament Manager Error:\n{value}"))
    }
}
