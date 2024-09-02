use std::error::Error;

use server::repository::{PlayerRepository, Repository};
use shared::equipment::weapon::Weapon;
use shared::inventory::{HasInventory, HasMutableInventory, Item, Items, MutableItems};
use shared::random::Random;
use shared::unique_entity::UniqueEntity;

use super::create_player;

#[test]
fn update_player_inventory() -> Result<(), Box<dyn Error>> {
    let repo = PlayerRepository::build()?;
    let mut player = create_player(&repo, vec![])?;
    let weapon = Weapon::random();
    
    assert!(player.inventory().items().len() < 1);
    player.inventory_mut().add_item(Item::Weapon(weapon));
    repo.update(player.uuid(), &player)?;

    let player = repo.get_by_uuid(player.uuid())?;
    assert!(player.inventory().items().len() > 0);
    Ok(())
}
