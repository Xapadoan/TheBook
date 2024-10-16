use std::path::PathBuf;

use shared::{tournament::{contestant::TournamentContestant, Tournament}, unique_entity::UniqueEntity, warrior::Warrior};
use uuid::Uuid;

use crate::{repository::{FileRepository, Repository}, tournament::manager::TournamentManager};

use super::PlayerAPIError;

pub fn register_contestant(player_uuid: &Uuid, tournament: &mut Tournament, warrior: &mut Warrior) -> Result<(), PlayerAPIError> {
    eprintln!("[WARN] Should use try_join! here");
    let repo: FileRepository<Warrior> = FileRepository::build(PathBuf::from("saves/warriors"))?;
    let manager = TournamentManager::build()?;
    manager.register_contestant(player_uuid, tournament, warrior)?;
    warrior.set_current_tournament(Some(tournament.uuid().clone()));
    repo.update(warrior.uuid(), &warrior)?;
    Ok(())
}
