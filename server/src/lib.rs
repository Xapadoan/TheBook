mod auth {
    mod signup;
    pub use signup::signup;
    mod session;
    pub use session::login_from_session;
    mod error;
    pub use error::AuthAPIError;
}

mod player {
    mod gen_random_warrior;
    pub use gen_random_warrior::gen_random_warrior;
    mod remove_warrior;
    pub use remove_warrior::remove_warrior;
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

pub mod api {
    pub mod auth {
        pub use crate::auth::{
            signup,
            login_from_session,
            AuthAPIError,
        };
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

    pub mod players {
        pub use crate::player::{
            gen_random_warrior,
            remove_warrior,
            PlayerAPIError,
        };
    }
}

use std::error::Error;

use tournament::manager::TournamentManager;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.run_tournaments {
        run_tournaments()?;
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
