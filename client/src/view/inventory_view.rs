use server::api;
use shared::{auth::Session, inventory::{HasInventory, Items, Item}, unique_entity::UniqueEntity};

use crate::{prompt::select_with_keys, show::ShowSelf};

use super::ViewError;

pub fn inventory_view(session: &Session) -> Result<(), ViewError> {
    let player = api::players::read(session.uuid())?;
    let options: Vec<&Item> = player.inventory().items().iter().collect();
    while let Some(_) = select_with_keys(
        "Select an item",
        &options,
        |item: &Item| { item.show_self() },
    )? {
        println!("Nothing to do with it")
    }
    Ok(())
}