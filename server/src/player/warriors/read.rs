use std::path::PathBuf;

use shared::{
    unique_entity::UniqueEntity,
    warrior::{MutableWarriorCollection, Warrior},
};
use uuid::Uuid;

use crate::{player::PlayerAPIError, repository::{FileRepository, PlayerRepository, Repository}};

pub fn read(player_uuid: &Uuid, warrior_uuid: &Uuid) -> Result<Warrior, PlayerAPIError> {
    let player_repo = PlayerRepository::build()?;
    let mut player = player_repo.get_by_uuid(player_uuid)?;
    let warrior = player.warriors_mut().iter().find(
        |warrior| { warrior.uuid() == warrior_uuid }
    );

    if warrior.is_none() {
        return Err(PlayerAPIError::new("Warrior not found"));
    }
    let warriors_repo: FileRepository<Warrior> = FileRepository::build(PathBuf::from("saves/warriors"))?;
    let warrior = warriors_repo.get_by_uuid(warrior_uuid)?;
    Ok(warrior)
}
