use std::path::PathBuf;

use shared::warrior::{MutableWarriorCollection, Warrior};
use uuid::Uuid;

use crate::repository::{FileRepository, PlayerRepository, Repository};

use super::PlayerAPIError;

pub fn remove_warrior(player_uuid: &Uuid, warrior_uuid: &Uuid) -> Result<(), PlayerAPIError> {
    let player_repo = PlayerRepository::build()?;
    let mut player = player_repo.get_by_uuid(player_uuid)?;
    match player.take_warrior(warrior_uuid) {
        Some(_) => {
            let warrior_repo: FileRepository<Warrior> = FileRepository::build(PathBuf::from("saves/warriors"))?;
            warrior_repo.delete(warrior_uuid)?;
        },
        None => eprintln!(
            "[WARN] warrior {} not found for player {}",
            warrior_uuid,
            player_uuid,
        ),
    }
    Ok(())
}