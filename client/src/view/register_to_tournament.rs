use server::api;
use shared::name::Name;
use shared::player::Player;
use shared::tournament::contestant::TournamentContestant;
use shared::unique_entity::UniqueEntity;
use shared::warrior::{Warrior, WarriorCollection};

use crate::prompt::{prompt_bool, swap_select_with_arrows};
use crate::show::{ShowSelf, CharacterSheet};

use super::ViewError;

pub fn register_to_tournament(player: Player) -> Result<(), ViewError> {
    let tournament = api::tournaments::playable_tournament()?;
    let send_warriors = prompt_bool(&format!(
        "A tournament, the {} will start soon, do you want to send warriors ?",
        tournament.name(),
    ))?;
    if !send_warriors {
        println!("Ok Bye !");
        return Ok(());
    }

    let mut warriors: Vec<&Warrior> = player.warriors()
        .iter()
        .filter(|w| w.current_tournament().is_none())
        .collect();
    let warrior = swap_select_with_arrows(
        "Select a warrior:",
        &mut warriors,
        | warrior| { CharacterSheet::new(warrior).show_self() }
    )?;
    if warrior.is_none() {
        return Ok(())
    }
    let warrior = warrior.unwrap();
    api::players::register_contestant(player.uuid(), tournament.uuid(), warrior.uuid())?;

    println!("{} registers for {}", warrior.name(), tournament.name());

    Ok(())
}
