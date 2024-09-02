use std::path::PathBuf;

use shared::{tournament::contestant::TournamentContestant, unique_entity::UniqueEntity, warrior::Warrior};
use uuid::Uuid;

use crate::{repository::{FileRepository, Repository}, tournament::manager::TournamentManager};

use super::PlayerAPIError;

pub fn register_contestant(player_uuid: &Uuid, tournament_uuid: &Uuid, warrior_uuid: &Uuid) -> Result<(), PlayerAPIError> {
    let repo: FileRepository<Warrior> = FileRepository::build(PathBuf::from("saves/warriors"))?;
    let mut warrior = repo.get_by_uuid(warrior_uuid)?;
    let manager = TournamentManager::build()?;
    manager.register_contestant(player_uuid, tournament_uuid, &mut warrior)?;
    warrior.set_current_tournament(Some(tournament_uuid.clone()));
    repo.update(warrior.uuid(), &warrior)?;
    Ok(())
}
