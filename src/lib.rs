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
    pub mod repository;
}
pub mod repository {
    pub mod main;
    pub mod file_repository;
}
pub mod random_dictionary;
pub mod client {
    mod prompt;
    pub mod main;
    mod select_warrior;
    mod player_creator;
    mod player_logger;
}

use std::error::Error;

use client::main::{handle_previous_tournaments, register_to_tournament, welcome_player};
use gen_random::GenRandom;
use tournament::manager::TournamentManager;
use warrior::Warrior;
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
        register_to_tournament(&mut player)?;
    }
    Ok(())
}

fn run_tournaments() -> Result<(), Box<dyn Error>> {
    let tournament_manager = TournamentManager::build()?;
    tournament_manager.run_tournaments()?;
    println!("Running tournaments");
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
