use std::error::Error;
use std::fmt::Display;

use shared::player::Player;
use shared::player::PlayerBuildError;
use shared::player::PlayerBuilder;
use shared::warrior::Warrior;
use uuid;

use crate::client::prompt::prompt;
use crate::repository::{
    FileRepository,
    PlayerDTOFile,
    PlayerRepository,
    Repository,
    RepositoryError,
};

use super::prompt::PromptError;

pub struct PlayerLogger {
    repo: PlayerRepository<FileRepository<PlayerDTOFile>, FileRepository<Warrior>>,
    player: Option<Player>,
}

impl PlayerLogger {
    pub fn build() -> Result<Self, PlayerLoggerError> {
        let repo = PlayerRepository::build()?;
        Ok(Self {
            repo,
            player: None,
        })
    }

    fn get_player_uuid(&self) -> Result<uuid::Uuid, PlayerLoggerError> {
        let str = prompt("Welcome back, enter your uuid:")?;
        let uuid = uuid::Uuid::parse_str(&str)?;
        Ok(uuid)
    }
}

impl PlayerBuilder for PlayerLogger {
    fn get_username(&mut self) -> Result<(), PlayerBuildError> {
        let uuid = self.get_player_uuid()?;
        println!("Client calls repo directly");
        let player = self.repo.get_by_uuid(&uuid)?;
        self.player = Some(player);
        Ok(())
    }

    fn get_display_name(&mut self) -> Result<(), PlayerBuildError> {
        Ok(())
    }

    fn get_warriors(&mut self) -> Result<(), PlayerBuildError> {
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

impl From<RepositoryError> for PlayerLoggerError {
    fn from(value: RepositoryError) -> Self {
        Self::new(format!("Repository Error:\n{value}"))
    }
}

impl From<PlayerLoggerError> for PlayerBuildError {
    fn from(value: PlayerLoggerError) -> Self {
        Self::new(format!("Player Logger Error:\n{value}"))
    }
}
