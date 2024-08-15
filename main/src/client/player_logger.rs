use std::error::Error;
use std::io;

use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;
use uuid::Uuid;

use crate::repository::file_repository::FileRepository;
use crate::repository::main::Repository;
use crate::repository::main::RepositoryError;
use crate::player::main::PlayerBuildError;
use crate::player::main::{Player, WarriorsManager};
use crate::player::main::PlayerBuildFinalStep;
use crate::player::main::PlayerBuildStepDisplayName;
use crate::player::main::{PlayerBuildStepUserName, PlayerBuilder};
use crate::player::repository::PlayerDTOFile;
use crate::player::repository::PlayerRepository;

pub struct PlayerLogger {
    repo: PlayerRepository<FileRepository<PlayerDTOFile>, FileRepository<Warrior>>,
    player: Option<Player>,
}

impl PlayerLogger {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let repo = PlayerRepository::build()?;
        Ok(Self {
            repo,
            player: None,
        })
    }
}

impl PlayerBuilder for PlayerLogger {
    fn get_username(&mut self) -> Result<PlayerBuildStepUserName, PlayerBuildError> {
        println!("Welcome back, enter your uuid:");
        let mut str = String::new();
        io::stdin().read_line(&mut str)?;
        let uuid = Uuid::parse_str(str.trim())?;
        let player = self.repo.get_by_uuid(&uuid)?;
        let username = String::from(player.username());
        self.player = Some(player);
        Ok(PlayerBuildStepUserName::new(username))
    }

    fn get_display_name(&mut self, previous_step: PlayerBuildStepUserName) -> Result<PlayerBuildStepDisplayName, PlayerBuildError> {
        let display_name = String::from(self.player.as_ref().unwrap().display_name());
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
