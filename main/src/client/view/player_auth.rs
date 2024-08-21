use shared::player::{Player, PlayerBuilder};

use crate::client::player_creator::PlayerCreator;
use crate::client::player_logger::PlayerLogger;
use crate::client::prompt::prompt_bool;
use crate::repository::{PlayerRepository, Repository};

use super::view_error::ViewError;

pub fn welcome_player() -> Result<Player, ViewError> {
    let repo = PlayerRepository::build()?;
    let player_has_account = prompt_bool("Do you already have an account ?")?;
    let player = if player_has_account {
        let mut builder = PlayerLogger::build()?;
        builder.get_username()?;
        builder.get_display_name()?;
        builder.get_warriors()?;
        let existing_player = builder.build();
        println!("Welcome back {} !", existing_player.display_name());
        existing_player
    } else  {
        let mut builder = PlayerCreator::new();
        builder.get_username()?;
        builder.get_display_name()?;
        builder.get_warriors()?;
        let new_player = builder.build();
        repo.create(&new_player)?;
        new_player
    };
    Ok(player)
}
