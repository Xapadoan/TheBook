use shared::{
    equipment::protection::Protection,
    inventory::{HasMutableInventory, Item, MutableItems},
    player::Player,
    unique_entity::UniqueEntity,
    warrior::{body::body_part::BodyPartKind, MutableWarriorCollection},
};
use uuid::Uuid;

use crate::{player::PlayerAPIError, repository::{PlayerRepository, Repository}};

use super::replace_protection::ReplaceProtection;

fn take_protection_from_player(player: &mut Player, inventory_slot: &Uuid) -> Option<Protection> {
    let new_protection = player.inventory_mut().remove_item(inventory_slot);
    if let None = new_protection {
        eprintln!(
            "[WARN] Item {} not found for player {}",
            inventory_slot,
            player.uuid(),
        );
        return None;
    }
    let new_protection = Protection::try_from(new_protection.unwrap());
    if let Err(e) = new_protection {
        eprintln!(
            "[WARN] Item {} couldn't be converted to a protection:\n{e}",
            inventory_slot,
        );
        return None;
    }
    Some(new_protection.unwrap())
}

pub fn equip_protection(player_uuid: &Uuid, warrior_uuid: &Uuid, body_part_kind: &BodyPartKind, inventory_slot: &Uuid) -> Result<(), PlayerAPIError> {
    let player_repo = PlayerRepository::build()?;
    let mut player = player_repo.get_by_uuid(&player_uuid)?;
    let new_protection = take_protection_from_player(&mut player, inventory_slot);
    if let None = new_protection { return Ok(()); }
    let new_protection = new_protection.unwrap();
    let warrior = player.warriors_mut().iter_mut().find(
        |warrior| { warrior.uuid() == warrior_uuid }
    );
    if let None = warrior {
        eprintln!(
            "[WARN] Warrior {} not found for player {}",
            warrior_uuid,
            player_uuid,
        )
    }
    let warrior = warrior.unwrap();
    let protection_to_store = warrior.replace_protection(body_part_kind, new_protection);
    if let Some(protection) = protection_to_store {
        player.inventory_mut().add_item(Item::Protection(protection));
    }
    player_repo.update(player.uuid(), &player)?;
    Ok(())
}
