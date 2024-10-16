use server::api;
use shared::auth::Session;
use shared::name::Name;
use shared::player::Player;
use shared::tournament::contestant::TournamentContestant;
use shared::unique_entity::UniqueEntity;
use shared::warrior::{Warrior, WarriorCollection};
use uuid::Uuid;

use crate::fetcher::ApiFetcher;
use crate::prompt::{prompt_bool, swap_select_with_arrows};
use crate::show::{ShowSelf, CharacterSheet};

use super::ViewError;

pub fn register_to_tournament(session: &Session) -> Result<(), ViewError> {
    let fetcher = ApiFetcher::new(session);
    let player: Player = fetcher.get("/player")?;
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
    fetcher.patch::<Vec<Uuid>, ()>(
        format!("/player/tournaments/{}/register", tournament.uuid().to_string()).as_str(),
        vec![warrior.uuid().clone()],
    )?;

    println!("{} registers for {}", warrior.name(), tournament.name());

    Ok(())
}
