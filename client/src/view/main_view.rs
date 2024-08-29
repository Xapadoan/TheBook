use std::fmt;

use server::api;
use shared::{auth::Session, unique_entity::UniqueEntity};

use crate::prompt::select_with_keys;

use super::{register_to_tournament, returning_warriors, ViewError};

enum MainViewChoice {
    ManageWarriors,
    ManageTournaments,
    // Quit,
}

const MAIN_VIEW_OPTIONS: [&'static MainViewChoice; 2] = [
    &MainViewChoice::ManageTournaments,
    &MainViewChoice::ManageWarriors,
    // &MainViewChoice::Quit,
];

impl fmt::Display for MainViewChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MainViewChoice::ManageTournaments => write!(f, "Manage Tournaments"),
            MainViewChoice::ManageWarriors => write!(f, "Manage Warriors"),
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
                        let player = api::players::read(session.uuid())?;
                        returning_warriors(player)?;
                        let player = api::players::read(session.uuid())?;
                        register_to_tournament(player)?;
                    },
                    MainViewChoice::ManageWarriors => {
                    },
                }
            }
            None => {
                println!("See you later !");
                return Ok(())
            }
        }
    }
}
