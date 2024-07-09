use std::error::Error;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::name::{HasName, Name};
use crate::repository::main::{Repository, UniqueEntity};
use crate::repository::file_repository::FileRepository;
use crate::warrior::Warrior;

use super::main::{Player, PlayerBuildFinalStep, PlayerBuildStepDisplayName, PlayerBuildStepUserName, PlayerBuilder, WarriorsManager};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerDTOFile {
    uuid: Uuid,
    username: Name,
    display_name: Name,
    warrior_ids: Vec<Uuid>,
}

impl From<&Player> for PlayerDTOFile {
    fn from(value: &Player) -> Self {
        let mut warrior_ids: Vec<Uuid> = vec![];
        for warrior in value.warriors() {
            warrior_ids.push(warrior.uuid().to_owned())
        }
        Self {
            uuid: value.uuid().clone(),
            username: value.username().clone(),
            display_name: value.name().clone(),
            warrior_ids,
        }
    }
}

impl UniqueEntity for PlayerDTOFile {
    fn uuid<'a>(&'a self) -> &'a Uuid {
        &self.uuid
    }
}

struct PlayerBuilderFromRepo {
    dto: PlayerDTOFile,
}

impl PlayerBuilderFromRepo {
    fn new(dto: PlayerDTOFile) -> Self {
        Self { dto }
    }
}

impl PlayerBuilder for PlayerBuilderFromRepo {
    fn get_username(&mut self) -> Result<PlayerBuildStepUserName, Box<dyn Error>> {
        Ok(PlayerBuildStepUserName::new(self.dto.username.clone()))
    }

    fn get_display_name(&mut self, previous_step: super::main::PlayerBuildStepUserName) -> Result<PlayerBuildStepDisplayName, Box<dyn Error>> {
        Ok(PlayerBuildStepDisplayName::new(self.dto.display_name.clone(), previous_step))
    }

    fn get_warriors(&mut self, previous_step: PlayerBuildStepDisplayName) -> Result<super::main::PlayerBuildFinalStep, Box<dyn Error>> {
        let warrior_repository = FileRepository::build(PathBuf::from("saves/warriors"))?;
        let mut warriors: Vec<Warrior> = vec![];
        for warrior_uuid in &self.dto.warrior_ids {
            let warrior: Warrior = warrior_repository.get_by_uuid(&warrior_uuid)?;
            warriors.push(warrior);
        }
        Ok(PlayerBuildFinalStep::new(warriors, previous_step))
    }
}

pub struct PlayerRepository<T: Repository<PlayerDTOFile>> {
    dto_repo: T
}

impl PlayerRepository<FileRepository> {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let dto_repo = FileRepository::build(PathBuf::from("saves/players"))?;
        Ok(Self { dto_repo })
    }
}

impl<T: Repository<PlayerDTOFile>> Repository<Player> for PlayerRepository<T> {
    fn create(&self, item: &Player) -> Result<(), Box<dyn Error>> {
        let cto = PlayerDTOFile::from(item);
        self.dto_repo.create(&cto)?;
        let warrior_repository = FileRepository::build(PathBuf::from("saves/warriors"))?;
        for warrior in item.warriors() {
            warrior_repository.create(warrior)?;
        }
        Ok(())
    }

    fn get_by_uuid(&self, uuid: &Uuid) -> Result<Player, Box<dyn Error>> {
        let dto = self.dto_repo.get_by_uuid(uuid)?;
        let mut builder = PlayerBuilderFromRepo::new(dto);
        let player = Player::build(&mut builder)?;
        Ok(player)
    }

    fn update(&self, uuid: &Uuid, item: &Player) -> Result<(), Box<dyn Error>> {
        let cto = PlayerDTOFile::from(item);
        self.dto_repo.update(uuid, &cto)?;
        let warrior_repository = FileRepository::build(PathBuf::from("saves/warriors"))?;
        for warrior in item.warriors() {
            warrior_repository.update(warrior.uuid(), warrior)?;
        }
        Ok(())
    }
}