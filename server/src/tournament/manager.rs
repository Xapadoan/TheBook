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

use crate::repository::sql_repository::tournaments::CreateTournamentSchema;
use crate::repository::{FileRepository, Repository, RepositoryCreate, RepositoryError, RepositoryList, RepositoryRead};
use crate::tournament::bot_player_builder::BotPlayerBuilder;
use crate::warrior::{WarriorManager, WarriorManagerError};

use super::auto_tournament::AutoTournament;

#[derive(Debug)]
pub struct TournamentManager<T>
where
    T: RepositoryCreate<Tournament, CreateTournamentSchema> +
        RepositoryRead<Tournament> +
        RepositoryList<Tournament>
{
    repo: T
}

// impl TournamentManager<FileRepository<Tournament>> {
//     pub fn build() -> Result<Self, TournamentManagerError> {
//         let repo: FileRepository<Tournament> = FileRepository::build(PathBuf::from("data/tournaments"))?;
//         Ok(Self { repo })
//     }
// }
impl<T> TournamentManager<T>
where
    T: RepositoryCreate<Tournament, CreateTournamentSchema> +
        RepositoryRead<Tournament> +
        RepositoryList<Tournament>
{
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
    pub async fn get_tournament(&self, tournament_uuid: &Uuid) -> Result<Tournament, TournamentManagerError> {
        let tournament = self.repo.read(tournament_uuid).await?;
        Ok(tournament)
    }
    pub async fn is_tournament_available(&self, tournament_uuid: &Uuid) -> bool {
        self.repo.read(tournament_uuid).await.is_ok()
    }
    async fn get_available_tournament(&self) -> Result<Option<Tournament>, TournamentManagerError> {
        let all_tournaments = self.repo.list().await?;
        for tournament in all_tournaments {
            if !tournament.is_full() {
                println!("Found tournament: {}", tournament.uuid());
                return Ok(Some(tournament))
            }
        }
        println!("Found no tournaments");
        Ok(None)
    }
    async fn build_random(&self) -> Result<Tournament, TournamentManagerError> {
        println!("Building random tournament");
        let tournament = Tournament::random();
        let tournament_schema = CreateTournamentSchema::from(tournament);
        let created_tournament = self.repo.create(&tournament_schema).await?;
        Ok(created_tournament)
    }
    pub async fn get_playable_tournament(&self) -> Result<Tournament, TournamentManagerError> {
        let available_tournament = self.get_available_tournament().await?;
        if available_tournament.is_some() {
            Ok(available_tournament.unwrap())
        } else {
            let new = self.build_random().await?;
            Ok(new)
        }
    }

    // pub async fn run_tournaments(&self) -> Result<(), TournamentManagerError> {
    //     panic!("Not implemented");
    //     eprintln!("[ERROR] No filter on tournaments");
    //     let available_tournaments = self.repo.list().await?;
    //     let warriors_manager = WarriorManager::build()?;
    //     for tournament in available_tournaments {
    //         eprintln!("[DEBUG] Running tournament {} ({})", tournament.name(), tournament.uuid().to_string());
    //         warriors_manager.apply_passive_healing(&tournament.contestants_ids()).await?;
    //         eprintln!("[WARN] Bot Player should be deleted no matter what happens, this is not the case");
    //         let bot_player_uuid = self.gen_bot_player(&mut tournament).await?;
    //         tournament.auto().await?;
    //         for (player_uuid, contestants) in tournament.contestants().clone() {
    //             let player_repository = PlayerRepository::build()?;
    //             let mut player = player_repository.read(&player_uuid)?;
    //             for warrior_uuid in contestants {
    //                 if let Some(inventory) = tournament.take_contestant_inventory(&warrior_uuid) {
    //                     player.inventory_mut().join(inventory);
    //                 }
    //             }
    //             player_repository.update(&player_uuid, &player)?;
    //         }
    //         self.delete_bot_player(&bot_player_uuid).await?;
    //         self.repo.delete(&uuid).await?;
    //     }
    //     Ok(())
    // }
}

// impl TournamentManager<FileRepository<Tournament>> {
    // pub async fn get_tournament(&self, tournament_uuid: &Uuid) -> Result<Tournament, TournamentManagerError> {
    //     let tournament = self.repo.read(tournament_uuid).await?;
    //     Ok(tournament)
    // }
    // async fn get_available_tournament(&self) -> Result<Option<Tournament>, TournamentManagerError> {
    //     let all_tournaments = self.repo.list().await?;
    //     for tournament in all_tournaments {
    //         if !tournament.is_full() {
    //             println!("Found tournament: {}", tournament.uuid());
    //             return Ok(Some(tournament))
    //         }
    //     }
    //     println!("Found no tournaments");
    //     Ok(None)
    // }

    // async fn build_random(&self) -> Result<Tournament, TournamentManagerError> {
    //     println!("Building random tournament");
    //     let tournament = Tournament::random();
    //     let tournament_schema = CreateTournamentSchema::from(tournament);
    //     let created_tournament = self.repo.create(&tournament_schema).await?;
    //     Ok(tournament)
    // }

    // async fn gen_bot_player(&self, tournament: &mut Tournament) -> Result<Uuid, TournamentManagerError> {
    //     panic!("Not implemented")
        // let mut bot_builder = BotPlayerBuilder::new(tournament);
        // bot_builder.build_username()?;
        // bot_builder.build_display_name()?;
        // bot_builder.build_warriors()?;
        // let bots_repo = PlayersRepository::build()?;
        // let bot = bot_builder.build();
        // for warrior in bot.warriors() {
        //     tournament.add_contestant(bot.uuid(), warrior)?;
        // }
        // bots_repo.create(&bot)?;
        // Ok(bot.uuid().clone())
    // }

    // async fn delete_bot_player(&self, bot_uuid: &Uuid) -> Result<(), TournamentManagerError> {
    //     panic!("Not implemented")
        // let bots_repo = PlayerRepository::build()?;
        // bots_repo.delete(bot_uuid)?;
        // Ok(())
    // }

    // pub async fn get_playable_tournament(&self) -> Result<Tournament, TournamentManagerError> {
    //     let available_tournament = self.get_available_tournament().await?;
    //     if available_tournament.is_some() {
    //         Ok(available_tournament.unwrap())
    //     } else {
    //         let new = self.build_random().await?;
    //         Ok(new)
    //     }
    // }

    // pub async fn register_contestant(
    //     &self,
    //     player_uuid: &Uuid,
    //     tournament: &mut Tournament,
    //     warrior: &Warrior,
    // ) -> Result<(), TournamentManagerError> {
    //     tournament.add_contestant(player_uuid, warrior)?;
    //     self.repo.update(tournament.uuid(), &tournament).await?;
    //     Ok(())
    // }

    // pub async fn run_tournaments(&self) -> Result<(), TournamentManagerError> {
    //     panic!("Not implemented")
    //     let tournaments_uuids = self.repo.list().await?;
    //     let warriors_manager = WarriorManager::build()?;
    //     for uuid in tournaments_uuids {
    //         let mut tournament = self.repo.read(&uuid).await?;
    //         eprintln!("[DEBUG] Running tournament {} ({})", tournament.name(), &uuid);
    //         warriors_manager.apply_passive_healing(&tournament.contestants_ids())?;
    //         let bot_player_uuid = self.gen_bot_player(&mut tournament).await?;
    //         tournament.auto()?;
    //         for (player_uuid, contestants) in tournament.contestants().clone() {
    //             let player_repository = PlayerRepository::build()?;
    //             let mut player = player_repository.read(&player_uuid)?;
    //             for warrior_uuid in contestants {
    //                 if let Some(inventory) = tournament.take_contestant_inventory(&warrior_uuid) {
    //                     player.inventory_mut().join(inventory);
    //                 }
    //             }
    //             player_repository.update(&player_uuid, &player)?;
    //         }
    //         self.delete_bot_player(&bot_player_uuid).await?;
    //         self.repo.delete(&uuid).await?;
    //     }
    //     Ok(())
    // }

//     pub async fn is_tournament_available(&self, tournament_uuid: &Uuid) -> bool {
//         self.repo.read(tournament_uuid).await.is_ok()
//     }
// }

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
