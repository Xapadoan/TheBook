use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use shared::{replay::{FightReplay, FightReplaySummary}, tournament::Tournament, warrior::Warrior};
use uuid::Uuid;

use crate::repository::{PlayerRepository, Repository, RepositoryError};

use super::manager::{ReplayManager, ReplayManagerError};

pub fn available_replays(player_uuid: &Uuid) -> Result<HashMap<Uuid, Vec<Uuid>>, ReplayAPIError> {
    let player_repo = PlayerRepository::build()?;
    let player = player_repo.get_by_uuid(player_uuid)?;
    let map = ReplayManager::map_warriors_to_replays(&player)?;
    Ok(map)
}

pub fn tournament_replay(tournament_uuid: &Uuid) -> Result<Tournament, ReplayAPIError> {
    let manager = ReplayManager::new(tournament_uuid);
    let tournament = manager.get_tournament_replay()?;
    Ok(tournament)
}

pub fn fight_summary_for_warrior(tournament_uuid: &Uuid, warrior_uuid: &Uuid, round_index: u8) -> Result<FightReplaySummary, ReplayAPIError> {
    let manager = ReplayManager::new(tournament_uuid);
    let summary = manager.get_fight_summary_for_warrior(warrior_uuid, round_index)?;
    Ok(summary)
}

pub fn fight_replay(tournament_uuid: &Uuid, fight_summary: &FightReplaySummary) -> Result<FightReplay, ReplayAPIError> {
    let manager = ReplayManager::new(tournament_uuid);
    let replay = manager.get_fight_replay(fight_summary)?;
    Ok(replay)
}

pub fn fight_warriors(tournament_uuid: &Uuid, fight_summary: &FightReplaySummary) -> Result<(Warrior, Warrior), ReplayAPIError> {
    eprintln!("[WARN] fight warrior should be merged with fight_replay");
    let manager = ReplayManager::new(tournament_uuid);
    let warriors = manager.get_fight_warriors(fight_summary)?;
    Ok(warriors)
}

#[derive(Debug)]
pub struct ReplayAPIError {
    message: String,
}

impl ReplayAPIError {
    pub fn new(message: &str) -> Self {
        Self { message: format!("Replay API Error:\n{message}") }
    }
}

impl Display for ReplayAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ReplayAPIError {}

impl From<ReplayManagerError> for ReplayAPIError {
    fn from(value: ReplayManagerError) -> Self {
        Self::new(&format!("Replay Manager Error:\n{value}"))
    }
}

impl From<RepositoryError> for ReplayAPIError {
    fn from(value: RepositoryError) -> Self {
        Self::new(&format!("Repository Error:\n{value}"))
    }
}