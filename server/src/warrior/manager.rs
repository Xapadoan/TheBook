use std::{error::Error, fmt::Display, path::PathBuf};

use shared::{health::MutablePassiveHealing, unique_entity::UniqueEntity, warrior::Warrior};
use uuid::Uuid;

use crate::repository::{FileRepository, Repository, RepositoryError};

pub struct WarriorManager {
    repo: FileRepository<Warrior>,
}

impl WarriorManager {
    pub fn build() -> Result<Self, WarriorManagerError> {
        let repo = FileRepository::build(PathBuf::from("saves/warriors"))?;
        Ok(Self { repo })
    }

    pub fn apply_passive_healing(&self, warrior_uuids: &[Uuid]) -> Result<(), WarriorManagerError> {
        for uuid in warrior_uuids {
            let mut warrior = self.repo.get_by_uuid(uuid)?;
            warrior.passive_heal();
            self.repo.update(warrior.uuid(), &warrior)?;
        }
        Ok(())
    }

    pub fn save(&self, warrior: &Warrior) -> Result<(), WarriorManagerError> {
        self.repo.update(warrior.uuid(), warrior)?;
        Ok(())
    }

    pub fn create(&self, warrior: &Warrior) -> Result<(), WarriorManagerError> {
        self.repo.create(warrior)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct WarriorManagerError {
    message: String,
}

impl WarriorManagerError {
    pub fn new(message: &str) -> Self {
        Self { message: format!("Warrior Manager Error:\n{message}") }
    }
}

impl Display for WarriorManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WarriorManagerError {}

impl From<RepositoryError> for WarriorManagerError {
    fn from(value: RepositoryError) -> Self {
        Self::new(&format!("Repository Error:\n{value}"))
    }
}
