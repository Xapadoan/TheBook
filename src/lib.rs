mod dice;
mod modifiers;
mod equipment;
mod tournament {
    pub mod main;
    mod fight {
        pub mod main;
        pub mod replay_data;
    }
    mod name;
    pub mod manager;
    mod round_replay;
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
use std::path::PathBuf;

use name::HasName;
use player::cli_creator::CliPlayerCreator;
use player::cli_logger::CliPlayerLogger;
use player::main::{Player, WarriorsManager};
use gen_random::GenRandom;
use player::repository::PlayerRepository;
use repository::main::{Repository, UniqueEntity};
use tournament::manager::TournamentManager;
use warrior::Warrior;
use warrior::weapon::{Weapon, GiveWeapon};
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
        player.replace_dead_warriors()?;
        play(&mut player)?;
    }
    Ok(())
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
