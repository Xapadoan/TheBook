use std::path::PathBuf;

use shared::unique_entity::UniqueEntity;
use shared::{random::Random, warrior::MutableWarriorCollection};
use shared::warrior::Warrior;
use uuid::Uuid;

use crate::repository::{FileRepository, PlayerRepository, Repository};
use crate::player::PlayerAPIError;

pub fn gen_random_warrior(player_uuid: &Uuid) -> Result<Warrior, PlayerAPIError> {
    let player_repo = PlayerRepository::build()?;
    let mut player = player_repo.get_by_uuid(player_uuid)?;
    let warrior = Warrior::random();
    let warrior_uuid = warrior.uuid().clone();
    let warrior_repo = FileRepository::build(PathBuf::from("saves/warriors"))?;
    warrior_repo.create(&warrior)?;
    player.warriors_mut().push(warrior);
    player_repo.update(player.uuid(), &player)?;
    let warrior = warrior_repo.get_by_uuid(&warrior_uuid)?;
    Ok(warrior)
}
