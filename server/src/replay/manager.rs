use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::{fs, io};
use std::path::PathBuf;

use shared::player::Player;
use shared::replay::turn_summary::TurnSummary;
use shared::replay::{FightReplay, FightReplaySummary};
use shared::tournament::contestant::TournamentContestant;
use shared::tournament::Tournament;
use shared::unique_entity::UniqueEntity;
use shared::warrior::{Warrior, WarriorCollection};
use uuid::Uuid;

use crate::repository::{FileRepository, Repository, RepositoryError};
use crate::tournament::manager::{TournamentManager, TournamentManagerError};

pub const REPLAY_ROOT_DIR: &'static str = "data/replays";

pub struct ReplayManager {
    tournament_uuid: Uuid,
}

impl ReplayManager {
    pub fn new(tournament_uuid: &Uuid) -> Self {
        Self {
            tournament_uuid: tournament_uuid.clone()
        }
    }
}

impl ReplayManager {
    pub fn get_tournament_replay(&self) -> Result<Tournament, ReplayManagerError> {
        let mut path = PathBuf::from(REPLAY_ROOT_DIR);
        path.push(self.tournament_uuid.to_string());
        path.push("initial_state.replay");
        let serialized_tournament = fs::read_to_string(path)?;
        let tournament: Tournament = serde_json::from_str(&serialized_tournament)?;
        Ok(tournament)
    }

    pub fn get_round_summary(&self, round_index: u8) -> Result<Vec<FightReplaySummary>, ReplayManagerError> {
        let mut path = PathBuf::from(REPLAY_ROOT_DIR);
        path.push(self.tournament_uuid.to_string());
        path.push(&format!("round_{round_index}"));
        path.push("summary.replay");
        let serialized_summaries = fs::read_to_string(path)?;
        let summaries: Vec<FightReplaySummary> = serde_json::from_str(&serialized_summaries)?;
        Ok(summaries)
    }

    pub fn get_fight_replay(&self, fight_summary: &FightReplaySummary) -> Result<FightReplay, ReplayManagerError> {
        let mut path = PathBuf::from(REPLAY_ROOT_DIR);
        path.push(self.tournament_uuid.to_string());
        path.push(fight_summary.replay_uuid().to_string());
        path.push("turns.replay");
        let serialized_turns = fs::read_to_string(path)?;
        let turns: Vec<TurnSummary> = serde_json::from_str(&serialized_turns)?;
        Ok(FightReplay::new(
            fight_summary.replay_uuid().clone(),
            fight_summary.blue_corner_uuid().clone(),
            fight_summary.red_corner_uuid().clone(),
            turns
        ))
    }

    pub fn get_fight_warriors(&self, fight_summary: &FightReplaySummary) -> Result<(Warrior, Warrior), ReplayManagerError> {
        let mut path = PathBuf::from(REPLAY_ROOT_DIR);
        path.push(self.tournament_uuid.to_string());
        path.push(fight_summary.replay_uuid().to_string());
        let warriors_repo: FileRepository<Warrior> = FileRepository::build(path)?;
        let blue_corner = warriors_repo.get_by_uuid(fight_summary.blue_corner_uuid())?;
        let red_corner = warriors_repo.get_by_uuid(fight_summary.red_corner_uuid())?;
        Ok((blue_corner, red_corner))
    }

    pub fn get_fight_summary_for_warrior(&self, warrior_uuid: &Uuid, round_index: u8) -> Result<FightReplaySummary, ReplayManagerError> {
        let round_summary = self.get_round_summary(round_index)?;
        for fight in round_summary {
            if fight.winner().is_some_and(|uuid| &uuid == warrior_uuid) {
                return Ok(fight);
            } else if fight.loser().is_some_and(|uuid| { &uuid == warrior_uuid }) {
                return Ok(fight);
            } else if fight.tie().is_some_and(|(uuid1, uuid2) | { &uuid1 == warrior_uuid || &uuid2 == warrior_uuid }) {
                return Ok(fight);
            }
        }
        return Err(ReplayManagerError::new(format!("Warrior with uuid {} was not found in round {}", warrior_uuid, round_index)))
    }

    pub fn map_warriors_to_replays(player: &Player) -> Result<HashMap<Uuid, Vec<Uuid>>, ReplayManagerError> {
        let mut map: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        for warrior in player.warriors() {
            if warrior.current_tournament().is_some() {
                let tournament_uuid = warrior.current_tournament().as_ref().unwrap();
                let tournament_manager = TournamentManager::build()?;
                if !tournament_manager.is_tournament_available(tournament_uuid) {
                    match map.get_mut(tournament_uuid) {
                        Some(vec) => { vec.push(warrior.uuid().clone()); },
                        None => { map.insert(tournament_uuid.clone(), vec![warrior.uuid().clone()]); },
                    }
                }
            }
        }
        Ok(map)
    }
}

#[derive(Debug)]
pub struct ReplayManagerError {
    message: String,
}

impl ReplayManagerError {
    fn new(message: String) -> Self {
        Self { message: format!("ReplayManager Error\n{message}") }
    }
}

impl Display for ReplayManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ReplayManagerError {}

impl From<io::Error> for ReplayManagerError {
    fn from(value: io::Error) -> Self {
        Self::new(format!("io::Error:\n{value}"))
    }
}

impl From<serde_json::Error> for ReplayManagerError {
    fn from(value: serde_json::Error) -> Self {
        Self::new(format!("serde_json::Error:\n{value}"))
    }
}

impl From<TournamentManagerError> for ReplayManagerError {
    fn from(value: TournamentManagerError) -> Self {
        Self::new(format!("Tournament Manager Error:\n{value}"))
        
    }
}

impl From<RepositoryError> for ReplayManagerError {
    fn from(value: RepositoryError) -> Self {
        Self::new(format!("Repository Error:\n{value}"))
        
    }
}
