use std::path::PathBuf;

use shared::equipment::weapon::OptionalMutableWeapon;
use shared::inventory::{HasMutableInventory, Item, MutableItems};
use shared::warrior::body::body_part::PROTECTABLE_BODY_PARTS;
use shared::warrior::{MutableWarriorCollection, Warrior};
use uuid::Uuid;

use crate::repository::{FileRepository, PlayerRepository, Repository};
use crate::player::PlayerAPIError;

use super::TakeProtections;

pub fn remove_warrior(player_uuid: &Uuid, warrior_uuid: &Uuid) -> Result<(), PlayerAPIError> {
    let player_repo = PlayerRepository::build()?;
    let mut player = player_repo.get_by_uuid(player_uuid)?;
    match player.take_warrior(warrior_uuid) {
        Some(_) => {
            let warrior_repo: FileRepository<Warrior> = FileRepository::build(PathBuf::from("saves/warriors"))?;
            let mut warrior = warrior_repo.get_by_uuid(warrior_uuid)?;
            let protections = warrior.take_protections(PROTECTABLE_BODY_PARTS.iter().collect());
            for protection in protections {
                player.inventory_mut().add_item(Item::Protection(protection));
            }
            if let Some(weapon) = warrior.weapon_mut().take() {
                player.inventory_mut().add_item(Item::Weapon(weapon));
            }
            warrior_repo.delete(warrior_uuid)?;
        },
        None => eprintln!(
            "[WARN] warrior {} not found for player {}",
            warrior_uuid,
            player_uuid,
        ),
    }
    player_repo.update(player_uuid, &player)?;
    Ok(())
}
