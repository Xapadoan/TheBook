use std::fmt;

use server::api;
use shared::{auth::Session, unique_entity::UniqueEntity};

use crate::prompt::select_with_keys;

use super::{shop_view, register_to_tournament, returning_warriors, warriors_view::warriors_view, ViewError};

enum MainViewChoice {
    ManageWarriors,
    ManageTournaments,
    ManageItems,
    // Quit,
}

const MAIN_VIEW_OPTIONS: [&'static MainViewChoice; 3] = [
    &MainViewChoice::ManageTournaments,
    &MainViewChoice::ManageWarriors,
    &MainViewChoice::ManageItems,
    // &MainViewChoice::Quit,
];

impl fmt::Display for MainViewChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MainViewChoice::ManageTournaments => write!(f, "Manage Tournaments"),
            MainViewChoice::ManageWarriors => write!(f, "Manage Warriors"),
            MainViewChoice::ManageItems => write!(f, "Manage Items"),
            // MainViewChoice::Quit => write!(f, "Quit"),
        }
    }
}

pub fn main_view(session: &Session) -> Result<(), ViewError> {
    loop {
        let choice = select_with_keys(
            "What do we do ?",
            &MAIN_VIEW_OPTIONS,
            |choice| { format!("{choice}") }
        )?;
        match choice {
            Some(c) => {
                match c {
                    MainViewChoice::ManageTournaments => {
                        returning_warriors(session)?;
                        let player = api::players::read(session.uuid())?;
                        register_to_tournament(player)?;
                    },
                    MainViewChoice::ManageWarriors => {
                        warriors_view(session)?;
                    },
                    MainViewChoice::ManageItems => {
                        shop_view::shop_view(session)?;
                    }
                }
            }
            None => {
                println!("See you later !");
                return Ok(())
            }
        }
    }
}
