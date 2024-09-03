use std::fmt;

use server::api;
use shared::auth::Session;
use shared::equipment::weapon::Weapon;
use shared::inventory::{HasInventory, Item};
use shared::name::Name;
use shared::player::Player;
use shared::tournament::contestant::TournamentContestant;
use shared::unique_entity::UniqueEntity;
use shared::warrior::{Warrior, WarriorCollection};
use uuid::Uuid;

use crate::character_sheet::CharacterSheet;
use crate::prompt::{select_with_arrows, select_with_keys};
use crate::show::ShowSelf;

use super::ViewError;

enum WarriorManagementChoice {
    ReplaceWeapon,
}

const CHOICES: [&'static WarriorManagementChoice; 1] = [
    &WarriorManagementChoice::ReplaceWeapon,
];

impl fmt::Display for WarriorManagementChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WarriorManagementChoice::ReplaceWeapon => write!(f, "Replace weapon"),
        }
    }
}

pub fn warriors_view(session: &Session) -> Result<(), ViewError> {
    loop {
        let player = api::players::read(session.uuid())?;
        let warriors: Vec<&Warrior> = player.warriors()
            .iter()
            .filter(|w| w.current_tournament().is_none())
            .collect();
        match select_with_arrows(
            "Select a warrior to manage:",
            &warriors,
            |warrior| { CharacterSheet::new(warrior).show_self() },
        )? {
            None => { return Ok(()) },
            Some(warrior) => {
                'action_selection: loop {
                    match select_with_keys(
                        &format!("What do you want to do to {}", warrior.name()),
                        &CHOICES,
                        |option| { format!("{option}") }
                    )? {
                        None => { break 'action_selection; },
                        Some(choice) => {
                            match choice {
                                WarriorManagementChoice::ReplaceWeapon => replace_weapon_view(&player, warrior)?,
                            }
                        },
                    }
                }
            }
        }

    }
    // while let Some(warrior) = selected_warrior {
    //     match select_with_keys(
    //         &format!("What do you want to do to {}", warrior.name()),
    //         &CHOICES,
    //         |option| { format!("{option}") }
    //     )? {
    //         Some(choice) => {
    //             match choice {
    //                 WarriorManagementChoice::ReplaceWeapon => replace_weapon_view(&player, warrior)?,
    //             }
    //             let selected_warrior_uuid = warrior.uuid().clone();
    //             player = api::players::read(session.uuid())?;
    //             warriors = pl
    //             selected_warrior = player.warriors()
    //                 .iter()
    //                 .find(|w| { *w.uuid() == selected_warrior_uuid });
    //         },
    //         None => {
    //             selected_warrior = select_with_arrows(
    //                 "Select a warrior to manage:",
    //                 &warriors,
    //                 |warrior| { CharacterSheet::new(warrior).show_self() },
    //             )?;
    //         }
    //     }
    // }
    // Ok(())
}

fn replace_weapon_view(player: &Player, warrior: &Warrior) -> Result<(), ViewError> {
    let available_weapons: Vec<(&Uuid, &Weapon)> = player.inventory().items()
        .iter()
        .filter_map(|(id, item)| {
            match item {
                Item::Weapon(weapon) => Some((id, weapon)),
                _ => None,
            }
        })
        .collect();
    let available_weapons_ref: Vec<&(&Uuid, &Weapon)> = available_weapons.iter().collect();
    let inventory_slot_uuid = match select_with_keys(
        &format!("Select a weapon to give to {}:", warrior.name()),
        &available_weapons_ref,
        |(_, weapon)| { weapon.show_self() },
    )? {
        Some((id, _)) => (*id).clone(),
        None => return Ok(()),
    };

    api::players::warriors::give_weapon(
        player.uuid(),
        warrior.uuid(),
        &inventory_slot_uuid,
    )?;
    Ok(())
}
