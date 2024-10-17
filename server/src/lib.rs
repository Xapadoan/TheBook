mod auth {
    mod signup;
    pub use signup::signup;
    mod session {
        pub mod manager;
    }
    mod error;
    pub use error::AuthAPIError;
    pub use session::manager::{SessionManager, SessionManagerError};
}

mod player {
    pub mod warriors {
        mod take_protections;
        pub use take_protections::TakeProtections;
        mod replace_protection;
        pub use replace_protection::ReplaceProtection;
    }
    mod error;
    pub use error::PlayerAPIError;
    mod read;
    pub use read::read_player;
    mod tournaments;
    pub use tournaments::register_contestant;
    mod shop;
    pub use shop::{buy_item, sell_item};
    mod manager;
    pub use manager::PlayerManager;
}

pub mod replay {
    mod fight_replay;
    pub use fight_replay::{FightReplayBuilder, FightReplayBuilderError};
    mod manager;
    pub use manager::ReplayManager;
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

mod http {
    mod app;
    pub use app::run_server;
    mod middlewares {
        mod auth;
        pub use auth::session_auth;
        mod get_player_warrior;
        pub use get_player_warrior::get_player_warrior;
        mod get_tournament;
        pub use get_tournament::get_tournament;
        mod get_replay;
        pub use get_replay::get_replay;
    }
    mod shop {
        mod routes;
        mod read;
        pub use routes::shop_routes;
    }
    mod player {
        mod routes;
        mod read;
        mod buy_item;
        mod sell_item;
        pub use routes::player_routes;
        mod tournaments {
            mod routes;
            mod new_replays;
            mod register_warriors;
            pub use routes::player_tournaments_routes;
        }
        mod warriors {
            mod routes;
            mod remove_from_replay;
            mod read;
            mod level_up;
            mod replace_weapon;
            mod replace_protection;
            mod remove_warrior;
            mod gen_random_warrior;
            pub use routes::player_warriors_routes;
        }
    }
    mod tournaments {
        mod routes;
        mod playable;
        pub use routes::tournaments_routes;
    }
    mod replays {
        mod routes;
        mod read;
        pub use routes::replay_routes;
    }
}

mod tournament {
    pub mod auto_tournament;
    mod fight;
    pub mod manager;
    pub mod public;
    mod bot_player_builder;
    mod fight_reward;
}

mod warrior {
    mod manager;
    pub use manager::{WarriorManager, WarriorManagerError};
}

pub mod repository {
    mod main;
    pub use main::{Repository, RepositoryError};
    mod file_repository;
    pub use file_repository::FileRepository;
    mod player_repository;
    pub use player_repository::{PlayerRepository, PlayerDTOFile};
}

mod shop {
    mod manager;
    pub use manager::ShopManager;
    mod error;
    pub use error::{ShopManagerError, ShopManagerErrorKind};
    mod public;
    pub use public::read_shop;
}

pub mod api {
    pub mod auth {
        pub use crate::auth::{
            signup,
            AuthAPIError,
        };
    }
    pub mod tournaments {
        pub use crate::tournament::public::TournamentAPIError;
    }

    pub mod replay {
        pub use crate::replay::{
            fight_summary_for_warrior,
            fight_replay,
            fight_warriors,
            ReplayAPIError,
        };
    }

    pub mod players {
        pub use crate::player::PlayerAPIError;
    }
}

use std::error::Error;

use http::run_server;
use shop::ShopManager;
use tournament::manager::TournamentManager;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.run_tournaments {
        run_tournaments()?;
    }
    if config.reset_shop {
        ShopManager::reset_shop()?;
    }
    if config.start_server {
        run_server();
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
    reset_shop: bool,
    start_server: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let mut config = Self {
            run_tournaments: false,
            reset_shop: false,
            start_server: false,
        };

        for arg in args {
            if arg == "--run-tournaments" {
                config.run_tournaments = true;
            } else if arg == "--reset-shop" {
                config.reset_shop = true;
            } else if arg == "--start-server" {
                config.start_server = true;
            }
        }
        config
    }
}
