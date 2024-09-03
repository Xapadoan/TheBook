use std::error::Error;

use server::{
    api::players::remove_warrior,
    repository::{PlayerRepository, Repository},
};
use shared::{
    equipment::weapon::OptionalMutableWeapon,
    inventory::HasInventory,
    random::Random,
    unique_entity::UniqueEntity,
    warrior::Warrior,
};

use crate::repository::create_player;

#[test]
fn warrior_items_go_back_to_player_inventory() -> Result<(), Box<dyn Error>> {
    let warrior = Warrior::random();
    assert!(warrior.weapon().is_some());
    let warrior_uuid = warrior.uuid().clone();
    let repo = PlayerRepository::build()?;
    let player = create_player(&repo, vec![warrior])?;
    let player_uuid = player.uuid().clone();
    remove_warrior(&player_uuid, &warrior_uuid)?;
    let player = repo.get_by_uuid(&player_uuid)?;

    assert!(player.inventory().items().len() > 0);
    Ok(())
}
