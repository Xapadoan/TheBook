use std::error::Error;
use std::fmt::Display;
use std::{fs, io};
use std::path::PathBuf;

use shared::replay::FightReplaySummary;
use uuid::Uuid;

use super::manager::REPLAY_ROOT_DIR;

pub struct RoundReplayBuilder {
    path: PathBuf,
    fights_summaries: Vec<FightReplaySummary>
}

impl RoundReplayBuilder {
    pub fn build(tournament_uuid: &Uuid, round_index: u8) -> Result<Self, RoundReplayBuilderError> {
        let mut path = PathBuf::from(REPLAY_ROOT_DIR);
        path.push(tournament_uuid.to_string());
        path.push(&format!("round_{}", round_index));
        fs::create_dir_all(&path)?;
        Ok(Self {
            path,
            fights_summaries: vec![],
        })
    }

    pub fn push_summary(&mut self, summary: FightReplaySummary) {
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
