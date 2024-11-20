use std::{error::Error, fmt::Display, path::PathBuf};

use shared::{health::MutablePassiveHealing, unique_entity::UniqueEntity, warrior::Warrior};
use uuid::Uuid;

use crate::repository::{sql_repository::warriors::{CreateWarriorSchema, UpdateWarriorSchema}, FileRepository, Repository, RepositoryCreate, RepositoryError, RepositoryRead, RepositoryUpdate};

pub struct WarriorManager<'a, T>
where T: RepositoryCreate<Warrior, CreateWarriorSchema>
{
    repo: &'a T,
}

impl<'a, T> WarriorManager<'a, T>
where T: RepositoryCreate<Warrior, CreateWarriorSchema> +
    RepositoryRead<Warrior> +
    RepositoryUpdate<Warrior, UpdateWarriorSchema>
{
    pub fn new(repo: &'a T) -> Self {
        Self { repo }
    }
    pub async fn create(&self, warrior: &CreateWarriorSchema) -> Result<(), WarriorManagerError> {
        self.repo.create(warrior).await?;
        Ok(())
    }
    pub async fn apply_passive_healing(&self, warrior_uuids: &[Uuid]) -> Result<(), WarriorManagerError> {
        for uuid in warrior_uuids {
            let mut warrior = self.repo.read(uuid).await?;
            warrior.passive_heal();
            self.repo.update(&warrior.uuid().clone(), &UpdateWarriorSchema::from(warrior)).await?;
        }
        Ok(())
    }
}

// impl WarriorManager {
//     pub fn build() -> Result<Self, WarriorManagerError> {
//         let repo = FileRepository::build(PathBuf::from("saves/warriors"))?;
//         Ok(Self { repo })
//     }

    // pub async fn apply_passive_healing(&self, warrior_uuids: &[Uuid]) -> Result<(), WarriorManagerError> {
    //     for uuid in warrior_uuids {
    //         let mut warrior = self.repo.read(uuid).await?;
    //         warrior.passive_heal();
    //         self.repo.update(warrior.uuid(), &warrior).await?;
    //     }
    //     Ok(())
    // }

    // pub async fn save(&self, warrior: &Warrior) -> Result<(), WarriorManagerError> {
    //     self.repo.update(warrior.uuid(), warrior).await?;
    //     Ok(())
    // }

    // pub async fn create(&self, warrior: &Warrior) -> Result<(), WarriorManagerError> {
    //     self.repo.create(warrior).await?;
    //     Ok(())
    // }
// }

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
