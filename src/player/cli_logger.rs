use std::error::Error;
use std::io;

use uuid::Uuid;

use crate::name::HasName;
use crate::repository::file_repository::FileRepository;
use crate::repository::main::Repository;
use crate::repository::main::RepositoryError;
use crate::repository::main::UniqueEntity;
use crate::warrior::Warrior;

use super::main::PlayerBuildError;
use super::main::{Player, WarriorsManager};
use super::main::PlayerBuildFinalStep;
use super::main::PlayerBuildStepDisplayName;
use super::main::{PlayerBuildStepUserName, PlayerBuilder};
use super::repository::PlayerDTOFile;
use super::repository::PlayerRepository;

pub struct CliPlayerLogger {
    repo: PlayerRepository<FileRepository<PlayerDTOFile>, FileRepository<Warrior>>,
    player: Option<Player>,
}

impl CliPlayerLogger {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let repo = PlayerRepository::build()?;
        Ok(Self {
            repo,
            player: None,
        })
    }
}

impl PlayerBuilder for CliPlayerLogger {
    fn get_username(&mut self) -> Result<PlayerBuildStepUserName, PlayerBuildError> {
        println!("Welcome back, enter your uuid:");
        let mut str = String::new();
        io::stdin().read_line(&mut str)?;
        let uuid = Uuid::parse_str(str.trim())?;
        let player = self.repo.get_by_uuid(&uuid)?;
        let username = player.username().clone();
        self.player = Some(player);
        Ok(PlayerBuildStepUserName::new(username))
    }

    fn get_display_name(&mut self, previous_step: PlayerBuildStepUserName) -> Result<PlayerBuildStepDisplayName, PlayerBuildError> {
        let display_name = self.player.as_ref().unwrap().name().clone();
        Ok(PlayerBuildStepDisplayName::new(display_name, previous_step))
    }

    fn get_warriors(&mut self, previous_step: PlayerBuildStepDisplayName) -> Result<PlayerBuildFinalStep, PlayerBuildError> {
        let mut player = self.player.take().unwrap();
        let mut warriors: Vec<Warrior> = vec![];
        while player.warriors().len() > 0 {
            let uuid = player.warriors()[0].uuid().clone();
            warriors.push(player.take_warrior(&uuid).unwrap())
        }
        Ok(PlayerBuildFinalStep::new(warriors, previous_step))
    }
}

impl From<io::Error> for PlayerBuildError {
    fn from(value: io::Error) -> Self {
        Self::new(format!("CLI Logger io::Error:\n{}", value))
    }
}

impl From<uuid::Error> for PlayerBuildError {
    fn from(value: uuid::Error) -> Self {
        Self::new(format!("CLI Logger uuid::Error:\n{}", value))
    }
}

impl From<RepositoryError> for PlayerBuildError {
    fn from(value: RepositoryError) -> Self {
        Self::new(format!("CLI Logger RepositoryError:\n{}", value))
    }
}
