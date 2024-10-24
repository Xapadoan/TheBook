mod prompt {
    mod error;
    pub use error::PromptError;
    mod prompt;
    pub use prompt::{
        prompt,
        prompt_bool,
    };
    mod select;
    pub use select::{
        select_with_keys,
        swap_select_with_keys,
        select_with_arrows,
        swap_select_with_arrows,
    };
}
mod fetcher;

mod view {
    mod view_error;
    pub use view_error::ViewError;
    mod finished_tournaments;
    pub use finished_tournaments::returning_warriors;
    mod player_auth;
    pub use player_auth::authenticate_player;
    mod register_to_tournament;
    pub use register_to_tournament::register_to_tournament;
    mod main_view;
    pub use main_view::main_view;
    mod shop_view;
    mod warriors_view;
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
        mod show_fighter;
        mod show_item;
        mod show_body_part;
        mod show_body;
    
        pub use main::{ShowSelf, ShowSelfExtended};
    }
    pub use show_self::{ShowSelf, ShowSelfExtended};

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
    pub use show_replay::ShowReplay;
    mod show_assault;
    mod show_resolution;
    mod show_self_critical_hit;
    mod show_fight_replay;
    pub use show_fight_replay::ShowWarriorFightReplay;
    mod show_turn_summary;
    mod character_sheet;
    pub use character_sheet::CharacterSheet;

    pub use main::*;
}

mod auth {
    mod session;
    pub use session::{read_session, store_session, SessionError};
}

pub fn run() -> Result<(), view::ViewError> {
    let mut session = view::authenticate_player()?;
    view::main_view(&mut session)?;
    Ok(())
}