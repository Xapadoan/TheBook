mod auth {
    mod signup;
    pub use signup::signup;
    mod session;
    pub use session::{login_from_session, SessionError};
}

mod player {
    mod gen_random_warrior;
    pub use gen_random_warrior::gen_random_warrior;
    mod error;
    pub use error::PlayerAPIError;
}

pub mod replay {
    mod fight_replay;
    pub use fight_replay::{FightReplayBuilder, FightReplayBuilderError};
    mod manager;
    mod tournament_replay;
    pub use tournament_replay::{TournamentReplayBuilder, TournamentReplayBuilderError};
    mod round_replay;
    pub use round_replay::{RoundReplayBuilder, RoundReplayBuilderError};
    mod public;
    pub use public::{
        available_replays,
        tournament_replay,
        fight_summary_for_warrior,
        fight_replay,
        // to remove after merge with fight replay
        fight_warriors,
        ReplayAPIError,
    };
}

mod tournament {
    pub mod auto_tournament;
    mod fight;
    pub use fight::{FightResult, FightResultKind};
    pub mod manager;
    pub mod public;
}

pub mod repository {
    mod main;
    pub use main::{Repository, RepositoryError};
    mod file_repository;
    pub use file_repository::FileRepository;
    mod player_repository;
    pub use player_repository::{PlayerRepository, PlayerDTOFile};
}

mod warrior {
    mod error;
    pub use error::WarriorAPIError;
    mod delete;
    pub use delete::delete_warrior;
}

pub mod client {
    mod prompt;
    mod main;
    pub use main::run;
    mod view {
        mod view_error;
        pub use view_error::ViewError;
        mod warrior_management;
        pub use warrior_management::returning_warriors;
        mod player_auth;
        pub use player_auth::welcome_player;
        mod register_to_tournament;
        pub use register_to_tournament::register_to_tournament;
    }
    mod show {
        mod main;
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

pub mod api {
    pub mod auth {
        pub use crate::auth::signup;
        pub use crate::auth::login_from_session;
        // remove this later
        pub use crate::auth::SessionError;
    }
    pub mod tournaments {
        pub use crate::tournament::public::{
            playable_tournament,
            register_contestant,
            remove_contestant,
            TournamentAPIError,
        };
    }

    pub mod replay {
        pub use crate::replay::{
            available_replays,
            tournament_replay,
            fight_summary_for_warrior,
            fight_replay,
            fight_warriors,
            ReplayAPIError,
        };
    }

    pub mod warriors {
        pub use crate::warrior::{
            delete_warrior,
            WarriorAPIError,
        };
    }

    pub mod players {
        pub use crate::player::{
            gen_random_warrior,
            PlayerAPIError,
        };
    }
}

use std::error::Error;

use tournament::manager::TournamentManager;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.run_tournaments {
        run_tournaments()?;
    } else {
        client::run()?;
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
