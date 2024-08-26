use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;

use shared::tournament::TournamentError;
use shared::{random::Random, tournament::Tournament};
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;
use uuid::Uuid;

use crate::repository::{FileRepository, Repository, RepositoryError};
use crate::warrior::{WarriorManager, WarriorManagerError};

use super::auto_tournament::AutoTournament;

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
                println!("Found tournament: {tournament_uuid}");
                return Ok(Some(tournament))
            }
        }
        println!("Found no tournaments");
        Ok(None)
    }

    fn build_random(&self) -> Result<Tournament, TournamentManagerError> {
        println!("Building random tournament");
        let tournament = Tournament::random();
        self.repo.create(&tournament)?;
        Ok(tournament)
    }

    fn gen_bots(&self, tournament: &mut Tournament) -> Result<Vec<Uuid>, TournamentManagerError> {
        let bots_number = tournament.max_contestants() - tournament.number_of_contestants();
        dbg!(bots_number);
        let mut i = 0;
        let bots_repo = FileRepository::build(PathBuf::from("saves/warriors"))?;
        let mut bots_uuids = vec![];
        while i < bots_number {
            let warrior = Warrior::random();
            bots_repo.create(&warrior)?;
            tournament.add_contestant(&warrior)?;
            self.repo.update(tournament.uuid(), tournament)?;
            bots_uuids.push(warrior.uuid().clone());
            i += 1;
        }
        dbg!(&bots_uuids);
        Ok(bots_uuids)
    }

    fn delete_bots(&self, bots: Vec<Uuid>) -> Result<(), TournamentManagerError> {
        let bots_repo: FileRepository<Warrior> = FileRepository::build(PathBuf::from("saves/warriors"))?;
        for bot_uuid in bots {
            bots_repo.delete(&bot_uuid)?;
        }
        Ok(())
    }

    pub fn get_playable_tournament(&self) -> Result<Tournament, TournamentManagerError> {
        let available_tournament = self.get_available_tournament()?;
        if available_tournament.is_some() {
            Ok(available_tournament.unwrap())
        } else {
            let new = self.build_random()?;
            Ok(new)
        }
    }

    pub fn register_contestant(&self, tournament_uuid: &Uuid, warrior: &Warrior) -> Result<(), TournamentManagerError> {
        let mut tournament = self.repo.get_by_uuid(tournament_uuid)?;
        tournament.add_contestant(warrior)?;
        self.repo.update(tournament.uuid(), &tournament)?;
        Ok(())
    }

    pub fn run_tournaments(&self) -> Result<(), TournamentManagerError> {
        let tournaments_uuids = self.repo.list()?;
        let warriors_manager = WarriorManager::build()?;
        for uuid in tournaments_uuids {
            let mut tournament = self.repo.get_by_uuid(&uuid)?;
            warriors_manager.apply_passive_healing(tournament.contestants_ids())?;
            let bots = self.gen_bots(&mut tournament)?;
            tournament.auto()?;
            self.delete_bots(bots)?;
            self.repo.delete(&uuid)?;
        }
        Ok(())
    }

    pub fn is_tournament_available(&self, tournament_uuid: &Uuid) -> bool {
        self.repo.get_by_uuid(tournament_uuid).is_ok()
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

impl From<WarriorManagerError> for TournamentManagerError {
    fn from(value: WarriorManagerError) -> Self {
        Self::new(&format!("Warrior Manager Error:\n{value}"))
    }
}
