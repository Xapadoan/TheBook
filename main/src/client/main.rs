use std::error::Error;
use std::path::PathBuf;

use shared::name::Name;
use shared::tournament::contestant::TournamentContestant;
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;

use crate::client::player_logger::PlayerLogger;
use crate::client::prompt::prompt_bool;
use crate::client::select_warrior::select_warrior;

use crate::player::main::{Player, WarriorsManager};
use crate::player::repository::PlayerRepository;
use crate::repository::file_repository::FileRepository;
use crate::repository::main::Repository;
use crate::tournament::{manager::TournamentManager, replay::manager::ReplayManager};


use super::player_creator::PlayerCreator;
use super::show::ShowAction;

pub fn register_to_tournament(player: &mut Player) -> Result<(), Box<dyn Error>> {
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

pub fn handle_previous_tournaments(player: &mut Player) -> Result<(), Box<dyn Error>> {
    let mut map = ReplayManager::map_warriors_to_replays(player)?;
    let warriors_repo = FileRepository::build(PathBuf::from("saves/warriors"))?;
    for (tournament_uuid, warriors) in map.iter_mut() {
        let replay_manager = ReplayManager::new(tournament_uuid);
        let tournament = replay_manager.get_tournament_replay()?;
        let show_replay = prompt_bool(&format!(
            "The {} tournament ended, {} of your warriors were there do you want to see what happened ?",
            tournament.name(),
            warriors.len(),
        ))?;
        if !show_replay {
            return Ok(());
        }

        'show_warrior: loop {
            let warrior = select_warrior(warriors)?;
            if warrior.is_none() {
                break 'show_warrior;
            }
            let warrior = warrior.unwrap();
            let number_of_rounds = tournament.number_of_rounds();
            show_warrior_tournament(warrior, number_of_rounds, &replay_manager)?;
            warrior.set_current_tournament(None);
            warriors_repo.update(warrior.uuid(), warrior)?;
        }
    }
    Ok(())
}

pub fn welcome_player() -> Result<Player, Box<dyn Error>> {
    let repo = PlayerRepository::build()?;
    let player_has_account = prompt_bool("Do you already have an account ?")?;
    let player = if player_has_account {
        let mut builder = PlayerLogger::build()?;
        let player = Player::build(&mut builder)?;
        println!("Welcome back {} !", player.display_name());
        player
    } else  {
        let mut builder = PlayerCreator::new();
        let tmp = Player::build(&mut builder)?;
        repo.create(&tmp)?;
        tmp
    };
    Ok(player)
}

fn show_warrior_tournament(
    warrior: &Warrior,
    number_of_rounds: usize,
    replay_manager: &ReplayManager,
) -> Result<(), Box<dyn Error>> {
    let mut round_index: u8 = 0;
    let mut warrior_lost = false;
    while !warrior_lost && usize::from(round_index) < number_of_rounds {
        let fight_summary = replay_manager.get_fight_summary_for_warrior(warrior, round_index)?;
        let prompt: String = if fight_summary.winner().is_some_and(|uuid| &uuid == warrior.uuid()) {
            format!(
                "{} won the {}th round, do you want to see a replay of the fight ?",
                warrior.name(),
                round_index,
            )
        } else if fight_summary.loser().is_some_and(|uuid| &uuid == warrior.uuid()) {
            warrior_lost = true;
            format!(
                "{} lost the {}th round, do you want to see a replay of the fight ?",
                warrior.name(),
                round_index,
            )
        } else {
            warrior_lost = true;
            format!(
                "Both warriors were eliminated during the {}th round, do you want to see a replay of the fight ?",
                round_index,
            )
        };
        let show_fight_replay = prompt_bool(&prompt)?;
        if show_fight_replay {
            dbg!("Showing fight replay...");
            dbg!(fight_summary.replay_uuid());
            let (assaults, warriors) = replay_manager.get_fight_replay(&fight_summary)?;
            for assault in assaults {
                let (assailant, victim) = if warriors.0.uuid() == assault.assailant_uuid() {
                    (&warriors.0, &warriors.1)
                } else {
                    (&warriors.1, &warriors.0)
                };
                println!("{}", assault.show_action(assailant, victim));
            }
        }
        round_index += 1;
    }
    Ok(())
}