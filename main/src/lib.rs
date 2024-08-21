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
pub mod player {
    pub mod main;
    pub mod repository;
}
pub mod repository {
    pub mod main;
    pub mod file_repository;
}
pub mod client {
    mod prompt;
    pub mod main;
    pub mod show {
        mod show_self {
            mod main;
            mod show_body_part_kind;
            mod show_protection;
            mod show_temporary_handicap;
            mod show_warrior;
            mod show_weapon;
    
            pub use main::ShowSelf;
        }
        pub use show_self::ShowSelf;

        mod show_replay {
            mod main;
            mod show_attack_clumsiness;
            mod show_attack_critical;
            mod show_attack_impossible;
            mod show_attack_missed;
            mod show_attack_success;
            mod show_parry_clumsiness;
            mod show_parry_critical;
            mod show_parry_impossible;

            pub use main::ShowReplay;
        }
        pub  use show_replay::ShowReplay;
        mod main;
        mod show_assault;
        mod show_resolution;
        mod show_self_critical_hit;
        mod show_fight_replay;
        pub use show_fight_replay::ShowFightReplay;
        mod show_turn_summary;

        pub use main::*;

    }
    mod select_warrior;
    mod player_creator;
    mod player_logger;
    mod character_sheet;
}

use std::error::Error;

use client::main::{handle_previous_tournaments, register_to_tournament, welcome_player};
use tournament::manager::TournamentManager;

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
