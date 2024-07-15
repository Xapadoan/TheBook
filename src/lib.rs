mod dice;
mod modifiers;
mod equipment;
mod tournament {
    pub mod main;
    mod fight;
    mod name;
    pub mod manager;
    pub mod replay {
        pub mod tournament_replay;
        pub mod round_replay;
        pub mod fight_replay;
        pub mod manager;
    }
}
mod warrior;
mod virtual_timer;
mod name;
mod gen_random;
pub mod player {
    pub mod main;
    pub mod cli_creator;
    pub mod cli_logger;
    pub mod repository;
}
pub mod repository {
    pub mod main;
    pub mod file_repository;
}
pub mod random_dictionary;

use std::error::Error;
use std::io;

use name::HasName;
use player::cli_creator::CliPlayerCreator;
use player::cli_logger::CliPlayerLogger;
use player::main::{Player, WarriorsManager};
use gen_random::GenRandom;
use player::repository::PlayerRepository;
use repository::main::{Repository, UniqueEntity};
use tournament::manager::TournamentManager;
use tournament::replay::manager::{ReplayManager, ReplayManagerError};
use tournament::replay::round_replay::FightSummary;
use uuid::Uuid;
use warrior::assault::show_action::ShowAction;
use warrior::assault::AssaultResult;
use warrior::{assault, Warrior};
use warrior::protection::{Protection, ProtectionKind, WearProtection};
use warrior::body::body_part::BodyPartKind;
use warrior::body::body_side::BodySide;

impl Warrior {
    fn wear_random_protection(&mut self, protection: Protection) {
        match protection.kind() {
            ProtectionKind::Armlet => self.wear_protection(protection, BodyPartKind::Arm(BodySide::gen_random())),
            ProtectionKind::Boot => self.wear_protection(protection, BodyPartKind::Foot(BodySide::gen_random())),
            ProtectionKind::ChainMail => self.wear_protection(protection, BodyPartKind::Torso),
            ProtectionKind::Gambeson => self.wear_protection(protection, BodyPartKind::Torso),
            ProtectionKind::Gauntlet => self.wear_protection(protection, BodyPartKind::Hand(BodySide::gen_random())),
            ProtectionKind::Greave => self.wear_protection(protection, BodyPartKind::Leg(BodySide::gen_random())),
            ProtectionKind::Helm => self.wear_protection(protection, BodyPartKind::Head),
            ProtectionKind::Jacket => self.wear_protection(protection, BodyPartKind::Torso),
            ProtectionKind::Plastron => self.wear_protection(protection, BodyPartKind::Torso),
        };
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.run_tournaments {
        run_tournaments()?;
    } else {
        let mut player = welcome_player()?;
        handle_previous_tournaments(&mut player)?;
        play(&mut player)?;
    }
    Ok(())
}

fn handle_previous_tournaments(player: &mut Player) -> Result<(), Box<dyn Error>> {
    let mut map = ReplayManager::map_warriors_to_replays(player)?;
    for (tournament_uuid, warriors) in map.iter_mut() {
        let replay_manager = ReplayManager::new(tournament_uuid);
        let tournament = replay_manager.get_tournament_replay()?;
        println!(
            "The {} tournament ended, {} of your warriors were there do you want to see what happened ? (Y / N)",
            tournament.name(),
            warriors.len(),
        );
        let mut user_response = String::new();
        io::stdin().read_line(&mut user_response)?;
        if user_response.trim().to_lowercase() != "y" {
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
        }
    }
    Ok(())
}

fn show_warrior_tournament(
    warrior: &Warrior,
    number_of_rounds: usize,
    replay_manager: &ReplayManager,
) -> Result<(), Box<dyn Error>> {
    let mut round_index: u8 = 0;
    let mut warrior_lost = false;
    let mut user_response = String::new();
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
        println!("{prompt}");
        user_response.clear();
        io::stdin().read_line(&mut user_response)?;
        if user_response.trim().to_lowercase() == "y" {
            let (assaults, warriors) = replay_manager.get_fight_replay(&fight_summary)?;
            for assault in assaults {
                let (assailant, victim) = if warriors.0.uuid() == assault.assailant_uuid() {
                    (&warriors.0, &warriors.1)
                } else {
                    (&warriors.1, &warriors.0)
                };
                assault.show(assailant, victim);
            }
        }
        round_index += 1;
    }
    Ok(())
}

fn select_warrior<'a>(warriors: &mut Vec<&'a mut Warrior>) -> Result<Option<&'a mut Warrior>, Box<dyn Error>> {
    if warriors.len() < 1 {
        return Ok(None)
    }
    println!("Select a warrior:");
    let mut i = 0;
    let mut user_response = String::new();
    while i < warriors.len() {
        println!("{}. {}", i + 1, warriors[i].name());
        i += 1;
    }
    println!("{}. Back", i + 1);
    user_response.clear();
    io::stdin().read_line(&mut user_response)?;
    let mut index: usize = user_response.trim().parse()?;
    index -= 1;
    if index > warriors.len() {
        return Ok(None)
    }
    let warrior = warriors.swap_remove(index);
    Ok(Some(warrior))
}

fn welcome_player() -> Result<Player, Box<dyn Error>> {
    let repo = PlayerRepository::build()?;
    println!("Do you already have an account ?\n (Y / N)");
    let mut user_response = String::new();
    io::stdin().read_line(&mut user_response)?;
    let player = if user_response.trim().to_lowercase() == "y" {
        let mut builder = CliPlayerLogger::build()?;
        Player::build(&mut builder)?
    } else  {
        let mut builder = CliPlayerCreator::new();
        let tmp = Player::build(&mut builder)?;
        repo.create(&tmp)?;
        tmp
    };
    Ok(player)
}

fn run_tournaments() -> Result<(), Box<dyn Error>> {
    let tournament_manager = TournamentManager::build()?;
    tournament_manager.run_tournaments()?;
    println!("Running tournaments");
    Ok(())
}

fn play(player: &mut Player) -> Result<(), Box<dyn Error>> {
    let tournament_manager = TournamentManager::build()?;
    let mut tournament = tournament_manager.get_playable_tournament()?;
    println!("A tournament, the {} will start soon, do you want to send warriors ? (Y/N)", tournament.name());
    let mut user_response = String::new();
    io::stdin().read_line(&mut user_response)?;
    if user_response.trim().to_lowercase() != "y" {
        println!("Ok Bye !");
        return Ok(());
    }

    println!("Select a warrior:");
    let mut i = 0;
    let warriors = player.warriors_mut();
    while i < warriors.len() {
        println!("{}. {}", i + 1, warriors[i].name());
        i += 1;
    }
    user_response.clear();
    io::stdin().read_line(&mut user_response)?;
    let mut index: usize = user_response.trim().parse()?;
    index -= 1;
    let warrior = &mut warriors[index];
    tournament_manager.register_contestant(warrior, &mut tournament)?;

    println!("{} registers for {}", warrior.name(), tournament.name());

    Ok(())
}

pub struct Config {
    run_tournaments: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        if args.len() < 2 {
            return Self { run_tournaments: false };
        }

        if args[1] == "--run-tournaments" {
            return Self { run_tournaments: true };
        } else {
            return Self { run_tournaments: false };
        }
    }
}
