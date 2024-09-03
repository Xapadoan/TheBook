use shared::{
    equipment::weapon::{OptionalMutableWeapon, Weapon},
    inventory::{HasMutableInventory, Item, MutableItems},
    player::Player,
    unique_entity::UniqueEntity,
    warrior::MutableWarriorCollection,
};
use uuid::Uuid;

use crate::{player::PlayerAPIError, repository::{PlayerRepository, Repository}};

pub fn give_weapon(player_uuid: &Uuid, warrior_uuid: &Uuid, inventory_slot: &Uuid) -> Result<(), PlayerAPIError> {
    let player_repo = PlayerRepository::build()?;
    let mut player = player_repo.get_by_uuid(&player_uuid)?;
    let new_weapon = take_weapon_from_player(&mut player, inventory_slot);
    if let None = new_weapon { return Ok(()); }
    let new_weapon = new_weapon.unwrap();
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
    let old_weapon = warrior.replace_weapon(new_weapon);
    if let Some(weapon) = old_weapon {
        player.inventory_mut().add_item(Item::Weapon(weapon));
    }
    player_repo.update(player.uuid(), &player)?;
    Ok(())
}

fn take_weapon_from_player(player: &mut Player, inventory_slot: &Uuid) -> Option<Weapon> {
    let new_weapon = player.inventory_mut().remove_item(inventory_slot);
    if let None = new_weapon {
        eprintln!(
            "[WARN] Item {} not found for player {}",
            inventory_slot,
            player.uuid(),
        );
        return None;
    }
    let new_weapon = Weapon::try_from(new_weapon.unwrap());
    if let Err(e) = new_weapon {
        eprintln!(
            "[WARN] Item {} couldn't be converted to a weapon:\n{e}",
            inventory_slot,
        );
        return None;
    }
    Some(new_weapon.unwrap())
}
