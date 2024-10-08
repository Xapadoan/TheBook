use std::error::Error;
use std::fmt::Display;
use std::{fs, io};
use std::path::PathBuf;

use shared::replay::turn_summary::TurnSummary;
use shared::warrior::Warrior;
use uuid::Uuid;

use crate::repository::{FileRepository, Repository, RepositoryError};

use super::manager::REPLAY_ROOT_DIR;

pub struct FightReplayBuilder<T: Repository<Warrior>> {
    replay_uuid: Uuid,
    turn_summaries: Vec<TurnSummary>,
    warriors_repo: T,
}

impl<T: Repository<Warrior>> FightReplayBuilder<T> {
    pub fn record_warriors_init_state(&self, blue: &Warrior, red: &Warrior) -> Result<(), FightReplayBuilderError> {
        self.warriors_repo.create(blue)?;
        self.warriors_repo.create(red)?;
        Ok(())
    }

    pub fn push_turn_summary(&mut self, turn_summary: TurnSummary) {
        self.turn_summaries.push(turn_summary)
    }

    pub fn replay_uuid(&self) -> &Uuid {
        &self.replay_uuid
    }
}

impl FightReplayBuilder<FileRepository<Warrior>> {
    pub fn build(tournament_uuid: &Uuid) -> Result<Self, FightReplayBuilderError> {
        let mut path = PathBuf::from(REPLAY_ROOT_DIR);
        path.push(tournament_uuid.clone().to_string());
        let replay_uuid = Uuid::new_v4();
        path.push(replay_uuid.to_string());
        let repo = FileRepository::build(path)?;
        Ok(Self { replay_uuid, warriors_repo: repo, turn_summaries: vec![] })
    }

    pub fn write_turn_summaries(&self) -> Result<(), FightReplayBuilderError> {
        let serialized_turn_summaries = serde_json::to_string(&self.turn_summaries)?;
        let path = self.warriors_repo.full_path("turns.replay");
        fs::write(&path, serialized_turn_summaries)?;
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
