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
        mod gen_random_warrior;
        pub use gen_random_warrior::gen_random_warrior;
        mod remove_warrior;
        pub use remove_warrior::remove_warrior;
        mod take_protections;
        pub use take_protections::TakeProtections;
        mod give_weapon;
        pub use give_weapon::give_weapon;
    }
    mod error;
    pub use error::PlayerAPIError;
    mod read;
    pub use read::read_player;
    mod tournaments;
    pub use tournaments::register_contestant;
    mod shop;
    pub use shop::{buy_item, sell_item};
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
    // pub use fight::{FightResult, FightResultKind};
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
    pub use public::{read_shop, ShopAPIError};
}

pub mod api {
    pub mod auth {
        pub use crate::auth::{
            signup,
            AuthAPIError,
        };
    }
    pub mod tournaments {
        pub use crate::tournament::public::{
            playable_tournament,
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
            warriors::{gen_random_warrior, remove_warrior},
            read_player as read,
            register_contestant,
            PlayerAPIError,
        };
        pub mod warriors {
            pub use crate::player::warriors::{
                give_weapon,
                gen_random_warrior as gen_random,
                remove_warrior as remove,
            };
        }
        pub mod shop {
            pub use crate::player::{
                buy_item,
                sell_item,
            };
        }
    }

    pub mod shop {
        pub use crate::shop::{
            read_shop,
            ShopAPIError,
        };
    }
}

use std::error::Error;

use shop::ShopManager;
use tournament::manager::TournamentManager;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.run_tournaments {
        run_tournaments()?;
    }
    if config.reset_shop {
        ShopManager::reset_shop()?;
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
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let mut config = Self {
            run_tournaments: false,
            reset_shop: false,
        };

        for arg in args {
            if arg == "--run-tournaments" {
                config.run_tournaments = true;
            } else if arg == "--reset-shop" {
                config.reset_shop = true;
            }
        }
        config
    }
}
