use server::api;
use shared::player::{Player, PlayerBuilder};
use shared::unique_entity::UniqueEntity;

use crate::auth::{read_session, PlayerCreator};

use super::view_error::ViewError;

pub fn welcome_player() -> Result<Player, ViewError> {
    let player = if let Some(session) = read_session()? {
        let existing_player = api::players::read(session.uuid())?;
        println!("Welcome back {} !", existing_player.display_name());
        existing_player
    } else {
        let mut builder = PlayerCreator::new();
        builder.build_username()?;
        builder.build_display_name()?;
        builder.build_warriors()?;
        let new_player = builder.build();
        new_player
    };
    Ok(player)
}
