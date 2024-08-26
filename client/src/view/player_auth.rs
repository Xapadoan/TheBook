use shared::player::{Player, PlayerBuilder};

use crate::auth::{PlayerCreator, PlayerLogger};
use crate::prompt::prompt_bool;

use super::view_error::ViewError;

pub fn welcome_player() -> Result<Player, ViewError> {
    let player_has_account = prompt_bool("Do you already have an account ?")?;
    let player = if player_has_account {
        let mut builder = PlayerLogger::new();
        builder.build_username()?;
        builder.build_display_name()?;
        builder.build_warriors()?;
        let existing_player = builder.build();
        println!("Welcome back {} !", existing_player.display_name());
        existing_player
    } else  {
        let mut builder = PlayerCreator::new();
        builder.build_username()?;
        builder.build_display_name()?;
        builder.build_warriors()?;
        let new_player = builder.build();
        new_player
    };
    Ok(player)
}
