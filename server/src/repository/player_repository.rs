use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use shared::inventory::{HasInventory, Inventory, Items, MutableItems};
use shared::player::{Player, PlayerBuildError, PlayerBuilder};
use shared::unique_entity::UniqueEntity;
use shared::warrior::{Warrior, WarriorCollection};
use uuid::Uuid;

use crate::repository::main::{Repository, RepositoryError};
use crate::repository::file_repository::FileRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerDTOFile {
    uuid: Uuid,
    username: String,
    display_name: String,
    warrior_ids: Vec<Uuid>,
    inventory: Inventory,
}

impl From<&Player> for PlayerDTOFile {
    fn from(value: &Player) -> Self {
        let mut warrior_ids: Vec<Uuid> = vec![];
        for warrior in value.warriors() {
            warrior_ids.push(warrior.uuid().clone())
        }
        let mut inventory = Inventory::new();
        for item in value.inventory().items() {
            inventory.add_item(item.clone());
        }
        Self {
            uuid: value.uuid().clone(),
            username: String::from(value.username()),
            display_name: String::from(value.display_name()),
            warrior_ids,
            inventory,
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
    warriors_repo: &'a T,
    warriors: Vec<Warrior>,
}

impl<'a, T: Repository<Warrior>> PlayerBuilderFromRepo<'a, T> {
    fn new(dto: PlayerDTOFile, warriors_repo: &'a T) -> Self {
        Self { dto, warriors_repo, warriors: vec![] }
    }
}

impl<'a, T: Repository<Warrior>> PlayerBuilder for PlayerBuilderFromRepo<'a, T> {
    fn build_username(&mut self) -> Result<(), PlayerBuildError> {
        Ok(())
    }
    fn build_display_name(&mut self) -> Result<(), PlayerBuildError> {
        Ok(())
    }
    fn build_warriors(&mut self) -> Result<(), PlayerBuildError> {
        for warrior_uuid in &self.dto.warrior_ids {
            let warrior: Warrior = self.warriors_repo.get_by_uuid(&warrior_uuid)?;
            self.warriors.push(warrior);
        }
        Ok(())
    }
    fn build_inventory(&mut self) -> Result<(), PlayerBuildError> {
        Ok(())
    }
    fn build(self) -> Player {
        Player::new(
            self.dto.uuid,
            self.dto.username,
            self.dto.display_name,
            self.warriors,
            self.dto.inventory,
        )
    }
}

pub struct PlayerRepository<T: Repository<PlayerDTOFile>, K: Repository<Warrior>> {
    dto_repo: T,
    warriors_repo: K
}

impl PlayerRepository<FileRepository<PlayerDTOFile>, FileRepository<Warrior>> {
    pub fn build() -> Result<Self, RepositoryError> {
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
        let dto = PlayerDTOFile::from(item);
        self.dto_repo.create(&dto)?;
        for warrior in item.warriors() {
            self.warriors_repo.create(warrior)?;
        }
        Ok(())
    }

    fn get_by_uuid(&self, uuid: &Uuid) -> Result<Player, RepositoryError> {
        let dto = self.dto_repo.get_by_uuid(uuid)?;
        let mut builder = PlayerBuilderFromRepo::new(dto, &self.warriors_repo);
        builder.build_username()?;
        builder.build_display_name()?;
        builder.build_warriors()?;
        Ok(builder.build())
    }

    fn update(&self, uuid: &Uuid, item: &Player) -> Result<(), RepositoryError> {
        let dto = PlayerDTOFile::from(item);
        self.dto_repo.update(uuid, &dto)?;
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
        Self::new(format!("PlayerBuildError:\n{}", value))
    }
}

impl From<RepositoryError> for PlayerBuildError {
    fn from(value: RepositoryError) -> Self {
        Self::new(format!("RepositoryError:\n{}", value))
    }
}
