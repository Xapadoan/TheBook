use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;

use shared::inventory::HasMutableInventory;
use shared::name::Name;
use shared::player::{PlayerBuildError, PlayerBuilder};
use shared::tournament::TournamentError;
use shared::{random::Random, tournament::Tournament};
use shared::unique_entity::UniqueEntity;
use shared::warrior::{Warrior, WarriorCollection};
use uuid::Uuid;

use crate::repository::{FileRepository, PlayerRepository, Repository, RepositoryError};
use crate::tournament::bot_player_builder::BotPlayerBuilder;
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
    pub fn get_tournament(&self, tournament_uuid: &Uuid) -> Result<Tournament, TournamentManagerError> {
        let tournament = self.repo.get_by_uuid(tournament_uuid)?;
        Ok(tournament)
    }
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

    fn gen_bot_player(&self, tournament: &mut Tournament) -> Result<Uuid, TournamentManagerError> {
        let mut bot_builder = BotPlayerBuilder::new(tournament);
        bot_builder.build_username()?;
        bot_builder.build_display_name()?;
        bot_builder.build_warriors()?;
        let bots_repo = PlayerRepository::build()?;
        let bot = bot_builder.build();
        for warrior in bot.warriors() {
            tournament.add_contestant(bot.uuid(), warrior)?;
        }
        bots_repo.create(&bot)?;
        Ok(bot.uuid().clone())
    }

    fn delete_bot_player(&self, bot_uuid: &Uuid) -> Result<(), TournamentManagerError> {
        let bots_repo = PlayerRepository::build()?;
        bots_repo.delete(bot_uuid)?;
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

    pub fn register_contestant(
        &self,
        player_uuid: &Uuid,
        tournament: &mut Tournament,
        warrior: &Warrior,
    ) -> Result<(), TournamentManagerError> {
        tournament.add_contestant(player_uuid, warrior)?;
        self.repo.update(tournament.uuid(), &tournament)?;
        Ok(())
    }

    pub fn run_tournaments(&self) -> Result<(), TournamentManagerError> {
        let tournaments_uuids = self.repo.list()?;
        let warriors_manager = WarriorManager::build()?;
        for uuid in tournaments_uuids {
            let mut tournament = self.repo.get_by_uuid(&uuid)?;
            eprintln!("[DEBUG] Running tournament {} ({})", tournament.name(), &uuid);
            warriors_manager.apply_passive_healing(&tournament.contestants_ids())?;
            let bot_player_uuid = self.gen_bot_player(&mut tournament)?;
            tournament.auto()?;
            for (player_uuid, contestants) in tournament.contestants().clone() {
                let player_repository = PlayerRepository::build()?;
                let mut player = player_repository.get_by_uuid(&player_uuid)?;
                for warrior_uuid in contestants {
                    if let Some(inventory) = tournament.take_contestant_inventory(&warrior_uuid) {
                        player.inventory_mut().join(inventory);
                    }
                }
                player_repository.update(&player_uuid, &player)?;
            }
            self.delete_bot_player(&bot_player_uuid)?;
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

impl From<PlayerBuildError> for TournamentManagerError {
    fn from(value: PlayerBuildError) -> Self {
        Self::new(&format!("Bot Player Error:\n{value}"))
    }
}
