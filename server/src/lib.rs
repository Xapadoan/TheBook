mod auth {
    mod signup;
    pub use signup::SignUp;
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
    mod tournaments;
    // pub use tournaments::register_contestant;
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
    mod auth {
        mod signup;
        mod routes;
        pub use routes::auth_routes;
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
        mod fight;
        pub use routes::replay_routes;
    }
}

mod tournament {
    pub mod auto_tournament;
    mod fight;
    pub mod manager;
    mod contestants_registerer;
    pub use contestants_registerer::ContestantsRegisterer;
    pub mod public;
    mod bot_player_builder;
    mod fight_reward;
    mod run_tournaments;
    pub use run_tournaments::run_tournaments;
}

mod warrior {
    mod manager;
    pub use manager::{WarriorManager, WarriorManagerError};
}

pub mod repository {
    mod main;
    pub use main::{Repository, RepositoryError, RepositoryCreate, RepositoryRead, RepositoryList, RepositoryUpdate, RepositoryDelete};
    mod file_repository;
    pub use file_repository::FileRepository;
    pub mod sql_repository {
        mod trx_repository;
        pub use trx_repository::TrxRepository;
        // Should remove and expose only top level repos
        mod query_builder;
        pub use query_builder::QueryBuilder;
        pub mod weapons {
            mod weapon_model;
            pub use weapon_model::{NewWeaponModel, WeaponKindColumnData, WeaponModel};
            mod weapon_schemas;
            pub use weapon_schemas::{CreateWeaponSchema, UpdateWeaponSchema};
            mod weapons_repository;
            pub use weapons_repository::{WeaponsPoolRepository, WeaponsQueryBuilder};
        }
        pub mod protections {
            mod protection_model;
            pub use protection_model::{NewProtectionModel, ProtectionKindColumnData, ProtectionModel};
            mod protection_schemas;
            pub use protection_schemas::{CreateProtectionSchema, UpdateProtectionSchema};
            mod protections_repository;
        }
        pub mod body_parts {
            mod body_part_model;
            pub use body_part_model::{BodyPartKindColumnData, BodyPartModel, BodyPartFinalModel};
            mod body_part_schemas;
            pub use body_part_schemas::{CreateBodyPartSchema, UpdateBodyPartSchema};
            mod body_parts_repository;
            pub use body_parts_repository::{BodyPartsRepository, BodyPartsQueryBuilder};
        }
        pub mod warriors {
            mod warrior_model;
            pub use warrior_model::{WarriorModel, WarriorFinalModel};
            mod warrior_schemas;
            pub use warrior_schemas::{CreateWarriorSchema, UpdateWarriorSchema};
            mod warriors_repository;
            pub use warriors_repository::{WarriorsRepository, WarriorsQueryBuilder};
        }
        pub mod players {
            mod player_model;
            pub use player_model::{PlayerModel, PlayerFinalModel};
            mod player_schemas;
            pub use player_schemas::{CreatePlayerSchema, UpdatePlayerSchema};
            mod players_repository;
            pub use players_repository::{PlayersQueryBuilder, PlayersRepository};
        }
        pub mod tournaments {
            mod tournament_model;
            pub use tournament_model::{TournamentModel, TournamentFinalModel};
            mod tournament_schemas;
            pub use tournament_schemas::{CreateTournamentSchema, UpdateTournamentSchema};
            mod tournaments_repository;
            pub use tournaments_repository::TournamentsRepository;
        }
        pub mod tournaments_warriors {
            mod tournament_warrior_model;
            pub use tournament_warrior_model::TournamentWarriorModel;
            mod tournament_warrior_schemas;
            pub use tournament_warrior_schemas::CreateTournamentWarriorSchema;
            mod tournaments_warriors_repository;
            pub use tournaments_warriors_repository::{TournamentsWarriorsQueryBuilder, TournamentsWarriorsRepository};
        }
        pub mod players_inventories {
            mod player_inventory_model;
            pub use player_inventory_model::{
                PlayerInventoryModel,
                PlayerInventoryFinalModel,
                PlayerInventoryJoinedSlotModel,
            };
            mod player_inventory_schemas;
            pub use player_inventory_schemas::{
                CreatePlayerInventorySchema,
                CreatePlayerInventorySlotSchema,
                UpdatePlayerInventorySchema,
            };
            mod players_inventories_repository;
            pub use players_inventories_repository::PlayersInventoriesRepository;
        }
        mod pool;
        pub use pool::gen_pool;
    }
}

mod shop {
    mod manager;
    pub use manager::ShopManager;
    mod error;
    pub use error::{ShopManagerError, ShopManagerErrorKind};
}

use std::error::Error;

use http::run_server;
use repository::sql_repository::{gen_pool, players::PlayersRepository, players_inventories::PlayersInventoriesRepository, tournaments::TournamentsRepository, warriors::WarriorsRepository};
use tournament::{manager::TournamentManager, run_tournaments};

pub async fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.run_tournaments {
        let db_pool = gen_pool().await;
        let tournaments_repository = TournamentsRepository::new(&db_pool);
        println!("Running tournaments");
        run_tournaments(
            &TournamentsRepository::new(&db_pool),
            &WarriorsRepository::new(&db_pool),
            &PlayersRepository::new(&db_pool),
            &PlayersInventoriesRepository::new(&db_pool),
        ).await?;
        eprintln!("[DEBUG] Tournaments run OK");
    }
    if config.start_server {
        run_server().await;
    }
    Ok(())
}

// async fn run_tournaments() -> Result<(), Box<dyn Error>> {
//     let db_pool = gen_pool().await;
//     let tournaments_repository = TournamentsRepository::new(&db_pool);
//     println!("Running tournaments");
//     run_tournaments(
//         TournamentsRepository::new(&db_pool),
//         WarriorsRepository::new(&db_pool),
//         PlayersRepository::new(&db_pool),
//         PlayersInventoriesRepository::new(&db_pool),
//     ).await?;
//     eprintln!("[DEBUG] Tournaments run OK");
//     Ok(())
// }

pub struct Config {
    run_tournaments: bool,
    start_server: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let mut config = Self {
            run_tournaments: false,
            start_server: false,
        };

        for arg in args {
            if arg == "--run-tournaments" {
                config.run_tournaments = true;
            } else if arg == "--start-server" {
                config.start_server = true;
            }
        }
        config
    }
}
