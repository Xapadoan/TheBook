use std::error::Error;
use std::fmt::Display;

use server::api::{
    players::PlayerAPIError,
    replay::ReplayAPIError,
    tournaments::TournamentAPIError,
};
use shared::player::PlayerBuildError;

use crate::player_logger::PlayerLoggerError;
use crate::prompt::PromptError;
use crate::select_warrior::SelectWarriorError;

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

impl From<ReplayAPIError> for ViewError {
    fn from(value: ReplayAPIError) -> Self {
        Self::new(format!("Replay API Error:\n{value}"))
    }
}

impl From<SelectWarriorError> for ViewError {
    fn from(value: SelectWarriorError) -> Self {
        Self::new(format!("Select Warrior Error:\n{value}"))
    }
}

impl From<TournamentAPIError> for ViewError {
    fn from(value: TournamentAPIError) -> Self {
        Self::new(format!("Tournament API Error:\n{value}"))
    }
}

impl From<PlayerAPIError> for ViewError {
    fn from(value: PlayerAPIError) -> Self {
        Self::new(format!("Player API Error:\n{value}"))
    }
}
