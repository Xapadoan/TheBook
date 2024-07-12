use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;

use crate::gen_random::GenRandom;
use crate::repository::main::{Repository, RepositoryError, UniqueEntity};
use crate::repository::file_repository::FileRepository;
use crate::warrior::Warrior;

use super::main::{Tournament, TournamentError};

#[derive(Debug)]
pub struct TournamentManager<T: Repository<Tournament>> {
    repo: T
}

impl TournamentManager<FileRepository<Tournament>> {
    pub fn build() -> Result<Self, TournamentManagerError> {
        let repo: FileRepository<Tournament> = FileRepository::build(PathBuf::from("data/tournaments"))?;
        Ok(Self { repo })
    }
}

impl<T: Repository<Tournament>> TournamentManager<T> {
    fn get_available_tournament(&self) -> Result<Option<Tournament>, TournamentManagerError> {
        let all_tournaments_uuids = self.repo.list()?;
        for tournament_uuid in all_tournaments_uuids {
            let tournament = self.repo.get_by_uuid(&tournament_uuid)?;
            if !tournament.is_full() {
                return Ok(Some(tournament))
            }
        }
        Ok(None)
    }

    fn build_random(&self) -> Result<Tournament, TournamentManagerError> {
        let tournament = Tournament::gen_random();
        let repo = FileRepository::build(PathBuf::from("data/tournaments"))?;
        repo.create(&tournament)?;
        Ok(tournament)
    }

    pub fn get_playable_tournament(&self) -> Result<Tournament, TournamentManagerError> {
        Ok(self.get_available_tournament()?.unwrap_or(self.build_random()?))
    }

    pub fn register_contestant(&self, warrior: &mut Warrior, tournament: &mut Tournament) -> Result<(), TournamentManagerError> {
        tournament.add_contestant(warrior)?;
        self.repo.update(tournament.uuid(), &tournament)?;
        warrior.set_current_tournament(Some(tournament.uuid().clone()));
        let warrior_repo: FileRepository<Warrior> = FileRepository::build(PathBuf::from("saves/warriors"))?;
        warrior_repo.update(warrior.uuid(), &warrior)?;
        Ok(())
    }

    pub fn run_tournaments(&self) -> Result<(), TournamentManagerError> {
        let tournaments_uuids = self.repo.list()?;
        for uuid in tournaments_uuids {
            let mut tournament = self.repo.get_by_uuid(&uuid)?;
            tournament.auto()?;
            tournament.release_warriors()?;
            self.repo.delete(&uuid)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct TournamentManagerError {
    message: String,
}

impl TournamentManagerError {
    pub fn new(message: &str) -> Self {
        Self { message: format!("Tournament Manager Error:\n{message}") }
    }
}

impl Display for TournamentManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for TournamentManagerError {}

impl From<RepositoryError> for TournamentManagerError {
    fn from(value: RepositoryError) -> Self {
        Self::new(&format!("Repository Error:\n{value}"))
    }
}

impl From<TournamentError> for TournamentManagerError {
    fn from(value: TournamentError) -> Self {
        Self::new(&format!("Tournament Error:\n{value}"))
    }
}
