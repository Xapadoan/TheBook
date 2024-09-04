use std::fmt;

use server::api;
use shared::{
    auth::Session,
    inventory::{GoldValue, HasInventory, Item},
    unique_entity::UniqueEntity,
};
use uuid::Uuid;

use crate::{prompt::select_with_keys, show::ShowSelf};

use super::ViewError;

enum ShopViewChoice {
    Buy,
    Sell,
}

const SHOP_VIEW_OPTIONS: [&'static ShopViewChoice; 2] = [
    &ShopViewChoice::Buy,
    &ShopViewChoice::Sell,
];

impl fmt::Display for ShopViewChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShopViewChoice::Buy => write!(f, "Buy"),
            Self::Sell => write!(f, "Sell"),
        }
    }
}

pub fn shop_view(session: &Session) -> Result<(), ViewError> {
    loop {
        let choice = select_with_keys(
            "Do you want to sell or buy ?",
            &SHOP_VIEW_OPTIONS,
            |option| { format!("{}", option) }
        )?;
        match choice {
            Some(c) => {
                match c {
                    ShopViewChoice::Buy => { buy_items_view(session)?; },
                    ShopViewChoice::Sell => { sell_items_view(session)?; },
                }
            }
            None => { return Ok(()) }
        }
    }
}

fn buy_items_view(session: &Session) -> Result<(), ViewError> {
    let mut player = api::players::read(session.uuid())?;
    let shop = api::shop::read_shop()?;
    let options: Vec<(&Uuid, &Item)> = shop.inventory().items()
        .iter()
        .collect();
    let options_as_reference: Vec<&(&Uuid, &Item)> = options.iter().collect();
    while let Some((id, _)) = select_with_keys(
        &format!("You have {} gold\nSelect an item:", player.inventory().gold()),
        &options_as_reference,
        |(_, item)| { format!("{} ({} gold)", item.show_self(), item.gold_value()) },
    )? {
        api::players::shop::buy_item(player.uuid(), id)?;
        player = api::players::read(session.uuid())?;
    }
    Ok(())
}

fn sell_items_view(session: &Session) -> Result<(), ViewError> {
    loop {
        let player = api::players::read(session.uuid())?;
        let options: Vec<(&Uuid, &Item)> = player.inventory().items()
            .iter()
            .collect();
        let options_as_reference: Vec<&(&Uuid, &Item)> = options.iter().collect();
        match select_with_keys(
            "Select an item to sell:",
            &options_as_reference,
            |(_, item)| {
                let value = item.gold_value() * 2 / 3;
                format!("{} ({} gold)", item.show_self(), value)
            },
        )? {
            Some((id, _)) => { api::players::shop::sell_item(player.uuid(), id)?; },
            None => { return Ok(()) },
        }
    }
}
