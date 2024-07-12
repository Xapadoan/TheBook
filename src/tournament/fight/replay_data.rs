use std::error::Error;
use std::fmt::Display;
use std::{fs, io};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repository::file_repository::FileRepository;
use crate::repository::main::{Repository, RepositoryError, UniqueEntity};
use crate::warrior::assault::AssaultResult;
use crate::warrior::Warrior;

#[derive(Debug, Serialize, Deserialize)]
pub struct FightReplayData {
    uuid: Uuid,
    blue: Warrior,
    red: Warrior,
    assaults: Vec<AssaultResult>
}

impl UniqueEntity for FightReplayData {
    fn uuid<'a>(&'a self) -> &'a Uuid {
        &self.uuid
    }
}

pub struct FightReplayBuilder<T: Repository<Warrior>> {
    replay_uuid: Uuid,
    assaults: Vec<AssaultResult>,
    warriors_repo: T,
}

impl<T: Repository<Warrior>> FightReplayBuilder<T> {
    pub fn record_warriors_init_state(&self, blue: &Warrior, red: &Warrior) -> Result<(), FightReplayBuilderError> {
        self.warriors_repo.create(blue)?;
        self.warriors_repo.create(red)?;
        Ok(())
    }

    pub fn push_assault(&mut self, assault: AssaultResult) {
        self.assaults.push(assault)
    }
}

impl FightReplayBuilder<FileRepository<Warrior>> {
    pub fn build(parent_path: &Path) -> Result<Self, FightReplayBuilderError> {
        let replay_uuid = Uuid::new_v4();
        let mut path = PathBuf::from(parent_path);
        path.push(replay_uuid.to_string());
        let repo = FileRepository::build(path)?;
        Ok(Self { replay_uuid, warriors_repo: repo, assaults: vec![] })
    }

    pub fn write_assaults(&self) -> Result<(), FightReplayBuilderError> {
        let serialized_assaults = serde_json::to_string(&self.assaults)?;
        fs::write("assault.replay", serialized_assaults)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct FightReplayBuilderError {
    message: String,
}

impl FightReplayBuilderError {
    fn new(message: String) -> Self {
        Self { message: format!("Fight Replay Build Error\n{message}") }
    }
}

impl Display for FightReplayBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for FightReplayBuilderError {}

impl From<RepositoryError> for FightReplayBuilderError {
    fn from(value: RepositoryError) -> Self {
        Self::new(format!("Repository Error:\n{value}"))
    }
}

impl From<io::Error> for FightReplayBuilderError {
    fn from(value: io::Error) -> Self {
        Self::new(format!("io::Error:\n{value}"))
    }
}

impl From<serde_json::Error> for FightReplayBuilderError {
    fn from(value: serde_json::Error) -> Self {
        Self::new(format!("io::Error\n{value}"))
    }
}
