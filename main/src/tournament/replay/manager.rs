use std::{collections::HashMap, error::Error, fmt::Display, fs, io, path::PathBuf};

use shared::assault::assault_summary::AssaultSummary;
use shared::tournament::contestant::TournamentContestant;
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;
use uuid::Uuid;

use crate::player::main::{Player, WarriorsManager};
use crate::repository::file_repository::FileRepository;
use crate::repository::main::{Repository, RepositoryError};
use crate::tournament::main::Tournament;
use crate::tournament::manager::{TournamentManager, TournamentManagerError};

use super::round_replay::FightSummary;

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

    pub fn get_round_summary(&self, round_index: u8) -> Result<Vec<FightSummary>, ReplayManagerError> {
        let mut path = PathBuf::from(REPLAY_ROOT_DIR);
        path.push(self.tournament_uuid.to_string());
        path.push(&format!("round_{round_index}"));
        path.push("summary.replay");
        let serialized_summaries = fs::read_to_string(path)?;
        let summaries: Vec<FightSummary> = serde_json::from_str(&serialized_summaries)?;
        Ok(summaries)
    }

    pub fn get_fight_replay(&self, fight_summary: &FightSummary) -> Result<(Vec<AssaultSummary>, (Warrior, Warrior)), ReplayManagerError> {
        let mut path = PathBuf::from(REPLAY_ROOT_DIR);
        path.push(self.tournament_uuid.to_string());
        path.push(fight_summary.replay_uuid().to_string());
        path.push("assaults.replay");
        let serialized_assaults = fs::read_to_string(path)?;
        let assaults: Vec<AssaultSummary> = serde_json::from_str(&serialized_assaults)?;
        let mut path = PathBuf::from(REPLAY_ROOT_DIR);
        path.push(self.tournament_uuid.to_string());
        path.push(fight_summary.replay_uuid().to_string());
        let warriors_repo: FileRepository<Warrior> = FileRepository::build(path)?;
        let warriors = if fight_summary.tie().is_some() {
            let (uuid1, uuid2) = fight_summary.tie().unwrap();
            let warrior1 = warriors_repo.get_by_uuid(&uuid1)?;
            let warrior2 = warriors_repo.get_by_uuid(&uuid2)?;
            (warrior1, warrior2)
        } else {
            let warrior1 = warriors_repo.get_by_uuid(fight_summary.winner().as_ref().unwrap())?;
            let warrior2 = warriors_repo.get_by_uuid(fight_summary.loser().as_ref().unwrap())?;
            (warrior1, warrior2)
        };
        Ok((assaults, warriors))
    }

pub fn get_fight_summary_for_warrior(&self, warrior: &Warrior, round_index: u8) -> Result<FightSummary, ReplayManagerError> {
        let round_summary = self.get_round_summary(round_index)?;
        for fight in round_summary {
            if fight.winner().is_some_and(|uuid| &uuid == warrior.uuid()) {
                return Ok(fight);
            } else if fight.loser().is_some_and(|uuid| { &uuid == warrior.uuid() }) {
                return Ok(fight);
            } else if fight.tie().is_some_and(|(uuid1, uuid2) | { &uuid1 == warrior.uuid() || &uuid2 == warrior.uuid() }) {
                return Ok(fight);
            }
        }
        return Err(ReplayManagerError::new(format!("Warrior with uuid {} was not found in round {}", warrior.uuid(), round_index)))
    }

    pub fn map_warriors_to_replays<'a>(player: &'a mut Player) -> Result<HashMap<Uuid, Vec<&'a mut Warrior>>, ReplayManagerError> {
        let mut map: HashMap<Uuid, Vec<&'a mut Warrior>> = HashMap::new();
        for warrior in player.warriors_mut() {
            if warrior.current_tournament().is_some() {
                let tournament_uuid = warrior.current_tournament().as_ref().unwrap();
                let tournament_manager = TournamentManager::build()?;
                if !tournament_manager.is_tournament_available(tournament_uuid) {
                    match map.get_mut(tournament_uuid) {
                        Some(vec) => { vec.push(warrior); },
                        None => { map.insert(tournament_uuid.clone(), vec![warrior]); },
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
