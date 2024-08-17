use std::{error::Error, fmt::Display, fs, io, path::PathBuf};

use serde::{Deserialize, Serialize};
use shared::unique_entity::UniqueEntity;
use uuid::Uuid;

use crate::tournament::fight::{FightResult, FightResultKind};

use super::manager::REPLAY_ROOT_DIR;

#[derive(Debug, Serialize, Deserialize)]
pub struct FightSummary {
    round_index: u8,
    replay_uuid: Uuid,
    winner: Option<Uuid>,
    loser: Option<Uuid>,
    tie: Option<(Uuid, Uuid)>,
}

impl FightSummary {
    pub fn winner(&self) -> &Option<Uuid> {
        &self.winner
    }

    pub fn loser(&self) -> &Option<Uuid> {
        &self.loser
    }

    pub fn tie(&self) -> &Option<(Uuid, Uuid)> {
        &self.tie
    }

    pub fn replay_uuid(&self) -> &Uuid {
        &self.replay_uuid
    }
}

pub struct RoundReplayBuilder {
    // tournament_uuid: Uuid,
    round_index: u8,
    path: PathBuf,
    fights_summaries: Vec<FightSummary>
}

impl RoundReplayBuilder {
    pub fn build(tournament_uuid: &Uuid, round_index: u8) -> Result<Self, RoundReplayBuilderError> {
        let mut path = PathBuf::from(REPLAY_ROOT_DIR);
        path.push(tournament_uuid.to_string());
        path.push(&format!("round_{}", round_index));
        fs::create_dir_all(&path)?;
        Ok(Self {
            // tournament_uuid: tournament_uuid.clone(),
            round_index,
            path,
            fights_summaries: vec![],
        })
    }

    pub fn push_summary(&mut self, replay_uuid: &Uuid, fight_result: &FightResult) {
        let (winner, loser, tie) = match fight_result.kind() {
            FightResultKind::Tie(warriors) => (
                None,
                None,
                Some((warriors.0.uuid().clone(), warriors.1.uuid().clone()))
            ),
            FightResultKind::Victory(fighters) => (
                Some(fighters.winner().uuid().clone()),
                Some(fighters.loser().uuid().clone()),
                None,
            )
        };
        let summary = FightSummary {
            round_index: self.round_index,
            replay_uuid: replay_uuid.clone(),
            winner,
            loser,
            tie
        };
        self.fights_summaries.push(summary);
    }

    pub fn write_summaries(&self) -> Result<(), RoundReplayBuilderError> {
        let serialized_summaries = serde_json::to_string(&self.fights_summaries)?;
        let mut path = self.path.clone();
        path.push("summary.replay");
        fs::write(&path, serialized_summaries)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct RoundReplayBuilderError {
    message: String,
}

impl RoundReplayBuilderError {
    fn new(message: String) -> Self {
        Self { message: format!("Round Replay Build Error\n{message}") }
    }
}

impl Display for RoundReplayBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RoundReplayBuilderError {}

impl From<io::Error> for RoundReplayBuilderError {
    fn from(value: io::Error) -> Self {
        Self::new(format!("io::Error:\n{value}"))
    }
}

impl From<serde_json::Error> for RoundReplayBuilderError {
    fn from(value: serde_json::Error) -> Self {
        Self::new(format!("serde_json::Error:\n{value}"))
    }
}