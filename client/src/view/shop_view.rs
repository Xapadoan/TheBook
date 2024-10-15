use std::fmt;

use shared::{
    auth::Session,
    inventory::{GoldValue, HasInventory, Item},
    player::Player,
    shop::Shop,
};
use uuid::Uuid;

use crate::{fetcher::ApiFetcher, prompt::select_with_keys, show::ShowSelfExtended};

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
    let fetcher = ApiFetcher::new(session);
    let mut player: Player = fetcher.get("/player")?;
    let shop: Shop = ApiFetcher::new(session).get("/shop")?;
    let options: Vec<(&Uuid, &Item)> = shop.inventory().items()
        .iter()
        .collect();
    let options_as_reference: Vec<&(&Uuid, &Item)> = options.iter().collect();
    while let Some((id, _)) = select_with_keys(
        &format!("You have {} gold\nSelect an item:", player.inventory().gold()),
        &options_as_reference,
        |(_, item)| { format!("{} ({} gold)", item.show_self_extended(), item.gold_value()) },
    )? {
        fetcher.patch::<(), Item>(
            format!("/player/shop/buy/{}", id.to_string()).as_str(),
            (),
        )?;
        player = fetcher.get("/player")?;
    }
    Ok(())
}

fn sell_items_view(session: &Session) -> Result<(), ViewError> {
    loop {
        let fetcher = ApiFetcher::new(session);
        let player: Player = fetcher.get("/player")?;
        let options: Vec<(&Uuid, &Item)> = player.inventory().items()
            .iter()
            .collect();
        let options_as_reference: Vec<&(&Uuid, &Item)> = options.iter().collect();
        match select_with_keys(
            &format!("You have {} gold\nSelect an item to sell:", player.inventory().gold()),
            &options_as_reference,
            |(_, item)| {
                let value = item.gold_value() * 2 / 3;
                format!("{} ({} gold)", item.show_self_extended(), value)
            },
        )? {
            Some((id, _)) => {
                fetcher.patch::<(), Player>(
                    format!("/player/shop/sell/{}", id.to_string()).as_str(),
                    (),
                )?;
            },
            None => { return Ok(()) },
        }
    }
}
