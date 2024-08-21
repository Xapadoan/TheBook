use shared::name::Name;
use shared::player::Player;
use shared::warrior::{MutableWarriorCollection, Warrior};

use crate::client::prompt::prompt_bool;
use crate::client::select_warrior::select_warrior;
use crate::tournament::manager::TournamentManager;

use super::ViewError;

pub fn register_to_tournament(player: &mut Player) -> Result<(), ViewError> {
    let tournament_manager = TournamentManager::build()?;
    let mut tournament = tournament_manager.get_playable_tournament()?;
    let send_warriors = prompt_bool(&format!(
        "A tournament, the {} will start soon, do you want to send warriors ?",
        tournament.name(),
    ))?;
    if !send_warriors {
        println!("Ok Bye !");
        return Ok(());
    }

    let mut warriors: Vec<&mut Warrior> = vec![];
    for warrior in player.warriors_mut() {
        warriors.push(warrior)
    }
    let warrior = select_warrior(&mut warriors)?;
    if warrior.is_none() {
        return Ok(())
    }
    let warrior = warrior.unwrap();
    tournament_manager.register_contestant(warrior, &mut tournament)?;

    println!("{} registers for {}", warrior.name(), tournament.name());

    Ok(())
}
