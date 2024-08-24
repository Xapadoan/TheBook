use std::error::Error;
use std::fmt::Display;

use server::api;
use shared::player::{Player, PlayerBuildError, PlayerBuilder};
use uuid;

use crate::prompt::prompt;

use super::prompt::PromptError;

pub struct PlayerLogger {
    player: Option<Player>,
}

impl PlayerLogger {
    pub fn new() -> PlayerLogger {
        Self {
            player: None,
        }
    }

    fn get_player_uuid(&self) -> Result<uuid::Uuid, PlayerLoggerError> {
        let str = prompt("Welcome back, enter your uuid:")?;
        let uuid = uuid::Uuid::parse_str(&str)?;
        Ok(uuid)
    }
}

impl PlayerBuilder for PlayerLogger {
    fn build_username(&mut self) -> Result<(), PlayerBuildError> {
        let uuid = self.get_player_uuid()?;
        let player = api::auth::login_from_session(&uuid)?;
        self.player = Some(player);
        Ok(())
    }

    fn build_display_name(&mut self) -> Result<(), PlayerBuildError> {
        Ok(())
    }

    fn build_warriors(&mut self) -> Result<(), PlayerBuildError> {
        Ok(())
    }

    fn build(self) -> Player {
        self.player.unwrap()
    }
}

#[derive(Debug)]
pub struct PlayerLoggerError {
    message: String,
}

impl PlayerLoggerError {
    pub fn new(message: String) -> Self {
        Self {
            message: format!("PlayerLoggerError:\n{message}")
        }
    }
}

impl Display for PlayerLoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PlayerLoggerError {}

impl From<PromptError> for PlayerLoggerError {
    fn from(value: PromptError) -> Self {
        Self::new(format!("Prompt Error:\n{value}"))
    }
}

impl From<uuid::Error> for PlayerLoggerError {
    fn from(value: uuid::Error) -> Self {
        Self::new(format!("Uuid Error:\n{value}"))
    }
}

impl From<api::auth::AuthAPIError> for PlayerLoggerError {
    fn from(value: api::auth::AuthAPIError) -> Self {
        Self::new(format!("Session Error:\n{value}"))
    }
}

impl From<PlayerLoggerError> for PlayerBuildError {
    fn from(value: PlayerLoggerError) -> Self {
        Self::new(format!("Player Logger Error:\n{value}"))
    }
}
