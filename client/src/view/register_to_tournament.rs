use server::api;
use shared::name::Name;
use shared::player::Player;
use shared::unique_entity::UniqueEntity;
use shared::warrior::{Warrior, WarriorCollection};

use crate::character_sheet::CharacterSheet;
use crate::prompt::{prompt_bool, select_with_arrows};
use crate::show::ShowSelf;

use super::ViewError;

pub fn register_to_tournament(player: &mut Player) -> Result<(), ViewError> {
    let tournament = api::tournaments::playable_tournament()?;
    let send_warriors = prompt_bool(&format!(
        "A tournament, the {} will start soon, do you want to send warriors ?",
        tournament.name(),
    ))?;
    if !send_warriors {
        println!("Ok Bye !");
        return Ok(());
    }

    let mut warriors: Vec<&Warrior> = vec![];
    for warrior in player.warriors() {
        warriors.push(warrior)
    }
    let warrior = select_with_arrows(
        "Select a warrior:",
        &mut warriors,
        | warrior| { CharacterSheet::new(warrior).show_self() }
    )?;
    if warrior.is_none() {
        return Ok(())
    }
    let warrior = warrior.unwrap();
    api::tournaments::register_contestant(tournament.uuid(), warrior.uuid())?;

    println!("{} registers for {}", warrior.name(), tournament.name());

    Ok(())
}