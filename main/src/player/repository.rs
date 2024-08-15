use std::error::Error;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;
use uuid::Uuid;

use crate::repository::main::{Repository, RepositoryError};
use crate::repository::file_repository::FileRepository;

use super::main::{Player, PlayerBuildError, PlayerBuildFinalStep, PlayerBuildStepDisplayName, PlayerBuildStepUserName, PlayerBuilder, WarriorsManager};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerDTOFile {
    uuid: Uuid,
    username: String,
    display_name: String,
    warrior_ids: Vec<Uuid>,
}

impl From<&Player> for PlayerDTOFile {
    fn from(value: &Player) -> Self {
        let mut warrior_ids: Vec<Uuid> = vec![];
        for warrior in value.warriors() {
            warrior_ids.push(warrior.uuid().clone())
        }
        Self {
            uuid: value.uuid().clone(),
            username: String::from(value.username()),
            display_name: String::from(value.display_name()),
            warrior_ids,
        }
    }
}

impl UniqueEntity for PlayerDTOFile {
    fn uuid<'a>(&'a self) -> &'a Uuid {
        &self.uuid
    }
}

struct PlayerBuilderFromRepo<'a, T: Repository<Warrior>> {
    dto: PlayerDTOFile,
    warriors_repo: &'a T
}

impl<'a, T: Repository<Warrior>> PlayerBuilderFromRepo<'a, T> {
    fn new(dto: PlayerDTOFile, warriors_repo: &'a T) -> Self {
        Self { dto, warriors_repo }
    }
}

impl<'a, T: Repository<Warrior>> PlayerBuilder for PlayerBuilderFromRepo<'a, T> {
    fn get_username(&mut self) -> Result<PlayerBuildStepUserName, PlayerBuildError> {
        Ok(PlayerBuildStepUserName::new(self.dto.username.clone()))
    }

    fn get_display_name(&mut self, previous_step: super::main::PlayerBuildStepUserName) -> Result<PlayerBuildStepDisplayName, PlayerBuildError> {
        Ok(PlayerBuildStepDisplayName::new(self.dto.display_name.clone(), previous_step))
    }

    fn get_warriors(&mut self, previous_step: PlayerBuildStepDisplayName) -> Result<PlayerBuildFinalStep, PlayerBuildError> {
        let mut warriors: Vec<Warrior> = vec![];
        for warrior_uuid in &self.dto.warrior_ids {
            let warrior: Warrior = self.warriors_repo.get_by_uuid(&warrior_uuid)?;
            warriors.push(warrior);
        }
        Ok(PlayerBuildFinalStep::new(warriors, previous_step))
    }
}

pub struct PlayerRepository<T: Repository<PlayerDTOFile>, K: Repository<Warrior>> {
    dto_repo: T,
    warriors_repo: K
}

impl PlayerRepository<FileRepository<PlayerDTOFile>, FileRepository<Warrior>> {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let dto_repo = FileRepository::build(PathBuf::from("saves/players"))?;
        let warriors_repo = FileRepository::build(PathBuf::from("saves/warriors"))?;
        Ok(Self { dto_repo, warriors_repo })
    }
}

impl<T: Repository<PlayerDTOFile>, K: Repository<Warrior>> Repository<Player> for PlayerRepository<T, K> {
    fn list(&self) -> Result<Vec<Uuid>, RepositoryError> {
        self.dto_repo.list()
    }

    fn create(&self, item: &Player) -> Result<(), RepositoryError> {
        let cto = PlayerDTOFile::from(item);
        self.dto_repo.create(&cto)?;
        for warrior in item.warriors() {
            self.warriors_repo.create(warrior)?;
        }
        Ok(())
    }

    fn get_by_uuid(&self, uuid: &Uuid) -> Result<Player, RepositoryError> {
        let dto = self.dto_repo.get_by_uuid(uuid)?;
        let mut builder = PlayerBuilderFromRepo::new(dto, &self.warriors_repo);
        let player = Player::build(&mut builder)?;
        Ok(player)
    }

    fn update(&self, uuid: &Uuid, item: &Player) -> Result<(), RepositoryError> {
        let cto = PlayerDTOFile::from(item);
        self.dto_repo.update(uuid, &cto)?;
        for warrior in item.warriors() {
            self.warriors_repo.update(warrior.uuid(), warrior)?;
        }
        Ok(())
    }

    fn delete(&self, uuid: &Uuid) -> Result<(), RepositoryError> {
        let dto = self.dto_repo.get_by_uuid(uuid)?;
        for warrior_id in dto.warrior_ids {
            self.warriors_repo.delete(&warrior_id)?;
        }
        self.dto_repo.delete(uuid)?;
        Ok(())
    }
}

impl From<PlayerBuildError> for RepositoryError {
    fn from(value: PlayerBuildError) -> Self {
        Self::new(format!("Repository PlayerBuildError:\n{}", value))
    }
}