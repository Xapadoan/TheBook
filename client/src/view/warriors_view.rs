use std::fmt;

use server::api;
use shared::auth::Session;
use shared::equipment::protection::{CanWearProtection, OptionalMutableProtection, Protection};
use shared::equipment::weapon::Weapon;
use shared::experience::{Experience, GainExperience};
use shared::inventory::{HasInventory, Item};
use shared::name::Name;
use shared::player::Player;
use shared::stats::StatKind;
use shared::tournament::contestant::TournamentContestant;
use shared::unique_entity::UniqueEntity;
use shared::warrior::body::body_part::{BodyPart, OptionalBodyPart, PROTECTABLE_BODY_PARTS};
use shared::warrior::body::HasBody;
use shared::warrior::{Warrior, WarriorCollection};
use uuid::Uuid;

use crate::fetcher::ApiFetcher;
use crate::prompt::{select_with_arrows, select_with_keys};
use crate::show::{CharacterSheet, ShowSelf, ShowSelfExtended};

use super::ViewError;

enum WarriorManagementChoice {
    ReplaceWeapon,
    EquipProtection,
    LevelUp,
}

const CHOICES: [&'static WarriorManagementChoice; 2] = [
    &WarriorManagementChoice::ReplaceWeapon,
    &WarriorManagementChoice::EquipProtection,
];

impl fmt::Display for WarriorManagementChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WarriorManagementChoice::ReplaceWeapon => write!(f, "Replace weapon"),
            WarriorManagementChoice::EquipProtection => write!(f, "Equip protection"),
            WarriorManagementChoice::LevelUp => write!(f, "Level Up"),
        }
    }
}

pub fn warriors_view(session: &Session) -> Result<(), ViewError> {
    let fetcher = ApiFetcher::new(session);
    loop {
        let player: Player = fetcher.get("/player")?;
        let warriors: Vec<&Warrior> = player.warriors()
            .iter()
            .filter(|w| w.current_tournament().is_none())
            .collect();
        let warrior_uuid = match select_with_arrows(
            "Select a warrior to manage:",
            &warriors,
            |warrior| { CharacterSheet::new(warrior).show_self_extended() },
        )? {
            None => return Ok(()),
            Some(warrior) => warrior.uuid(),
        };
        'action_selection: loop {
            let warrior: Warrior = fetcher.get(
                format!("/player/warriors/{}", warrior_uuid.to_string()).as_str()
            )?;
            let mut choices = CHOICES.to_vec();
            if warrior.can_level_up() {
                choices.push(&WarriorManagementChoice::LevelUp);
            }
            match select_with_keys(
                &format!("What do you want to do to {}", warrior.name()),
                &choices,
                |option| { format!("{option}") }
            )? {
                None => { break 'action_selection; },
                Some(choice) => {
                    match choice {
                        WarriorManagementChoice::ReplaceWeapon => replace_weapon_view(&player, &warrior)?,
                        WarriorManagementChoice::EquipProtection => equip_protection_view(&player, &warrior)?,
                        WarriorManagementChoice::LevelUp => level_up_view(&player, &warrior)?,
                    }
                },
            }
        }
    }
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

fn equip_protection_view(player: &Player, warrior: &Warrior) -> Result<(), ViewError> {
    let available_body_parts: Vec<&BodyPart> = PROTECTABLE_BODY_PARTS.iter()
        .filter_map(|kind| {
            match warrior.body().body_part(kind) {
                None => None,
                Some(part) => Some(part),
            }})
        .collect();
    let body_part = match select_with_keys(
        &format!("Select a limb to protect:"),
        &available_body_parts,
        |part| {
            match part.protection() {
                None => part.kind().show_self(),
                Some(protection) => format!("{} ({})", part.kind().show_self(), protection.show_self())
            }
        }
    )? {
        Some(part) => part,
        None => { return Ok(()) },
    };
    let available_protections: Vec<(&Uuid, &Protection)> = player.inventory().items()
        .iter()
        .filter_map(|(id, item)| {
            match item {
                Item::Protection(protection) => if body_part.can_wear_protection(protection) {
                    Some((id, protection))
                } else {
                    None
                }
                _ => None,
            }
        })
        .collect();
    let available_protections_ref: Vec<&(&Uuid, &Protection)> = available_protections.iter().collect();
    let inventory_slot_uuid = match select_with_keys(
        &format!("Select a protection to give to {}:", warrior.name()),
        &available_protections_ref,
        |(_, protection)| { protection.show_self() },
    )? {
        Some((id, _)) => (*id).clone(),
        None => return Ok(()),
    };

    api::players::warriors::equip_protection(
        player.uuid(),
        warrior.uuid(),
        body_part.kind(),
        &inventory_slot_uuid,
    )?;
    Ok(())
}

fn level_up_view(player: &Player, warrior: &Warrior) -> Result<(), ViewError> {
    let sheet = CharacterSheet::new(warrior);
    let possible_stats = if (warrior.level() + 1) % 2 == 0 {
        [&StatKind::Courage, &StatKind::Dexterity, &StatKind::Strength].to_vec()
    } else {
        [&StatKind::Attack, &StatKind::Parry].to_vec()
    };
    let prompt_str = format!("{}\nWhat stat do you want to update ?", sheet.show_self());
    let stat_to_increment = match select_with_keys(
        &prompt_str,
        &possible_stats,
        |stat| {
            match stat {
                &StatKind::Attack => "Attack".to_string(),
                &StatKind::Parry => "Parry".to_string(),
                &StatKind::Courage => "Courage".to_string(),
                &StatKind::Dexterity => "Dexterity".to_string(),
                &StatKind::Strength => "Strength".to_string(),
            }
        }
    )? {
        Some(stat) => stat,
        None => return Ok(()),
    };
    api::players::warriors::level_up(
        player.uuid(),
        warrior.uuid(),
        stat_to_increment,
    )?;
    Ok(())
}
