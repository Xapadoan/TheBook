use std::error::Error;
use std::fmt::Display;
use std::io;
use std::fs;
use std::path::PathBuf;

use shared::tournament::Tournament;
use uuid::Uuid;

use super::manager::REPLAY_ROOT_DIR;

pub struct TournamentReplayBuilder {
    path: PathBuf,
}

impl TournamentReplayBuilder {
    pub fn build(tournament_uuid: &Uuid) -> Result<Self, TournamentReplayBuilderError> {
        let mut path = PathBuf::from(REPLAY_ROOT_DIR);
        path.push(tournament_uuid.to_string());
        fs::create_dir_all(&path)?;
        Ok(Self {
            path,
        })
    }

    pub fn write_tournament_init_state(&self, tournament: &Tournament) -> Result<(), TournamentReplayBuilderError> {
        let mut path = self.path.clone();
        path.push("initial_state.replay");
        let serialized_tournament = serde_json::to_string(tournament)?;
        fs::write(&path, serialized_tournament)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct TournamentReplayBuilderError {
    message: String,
}

impl TournamentReplayBuilderError {
    fn new(message: String) -> Self {
        Self { message: format!("Tournament Replay Build Error\n{message}") }
    }
}

impl Display for TournamentReplayBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for TournamentReplayBuilderError {}

impl From<io::Error> for TournamentReplayBuilderError {
    fn from(value: io::Error) -> Self {
        Self::new(format!("io::Error:\n{value}"))
    }
}

impl From<serde_json::Error> for TournamentReplayBuilderError {
    fn from(value: serde_json::Error) -> Self {
        Self::new(format!("serde_json::Error:\n{value}"))
    }
}