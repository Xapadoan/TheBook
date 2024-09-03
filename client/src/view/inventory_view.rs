use server::api;
use shared::{
    auth::Session,
    inventory::{HasInventory, Item},
    unique_entity::UniqueEntity,
};
use uuid::Uuid;

use crate::{prompt::select_with_keys, show::ShowSelf};

use super::ViewError;

pub fn inventory_view(session: &Session) -> Result<(), ViewError> {
    let player = api::players::read(session.uuid())?;
    let options: Vec<(&Uuid, &Item)> = player.inventory().items()
        .iter()
        .collect();
    let options_as_reference: Vec<&(&Uuid, &Item)> = options.iter().collect();
    while let Some(_) = select_with_keys(
        "Select an item",
        &options_as_reference,
        |(slot_uuid, item)| { item.show_self() },
    )? {
        println!("Nothing to do with it")
    }
    Ok(())
}
