use server::api;
use shared::{player::{Player, PlayerBuildError, PlayerBuilder}, unique_entity::UniqueEntity};

use crate::prompt::{prompt, PromptError};

use super::session::store_session;

pub struct PlayerCreator {
    username: Option<String>,
    display_name: Option<String>,
    player: Option<Player>,
}

impl PlayerCreator {
    pub fn new() -> Self {
        Self { username: None, display_name: None, player: None }
    }
}

impl PlayerBuilder for PlayerCreator {
    fn build_username(&mut self) -> Result<(), PlayerBuildError> {
        let username = prompt("Choose a username:")?;
        self.username = Some(username);
        Ok(())
    }

    fn build_display_name(&mut self) -> Result<(), PlayerBuildError> {
        let display_name = prompt("Choose a display name:")?;
        self.display_name = Some(display_name);
        Ok(())
    }

    fn build_warriors(&mut self) -> Result<(), PlayerBuildError> {
        let username = match &self.username {
            Some(u) => String::from(u),
            None => {
                return Err(PlayerBuildError::new(
                    "build_username wasn't called".to_string())
                );
            }
        };
        let display_name = match &self.display_name {
            Some(u) => String::from(u),
            None => {
                return Err(PlayerBuildError::new(
                    "build_display_name wasn't called".to_string())
                );
            }
        };
        let session = api::auth::signup(username, display_name)?;
        store_session(&session)?;
        let player = api::players::read(session.uuid()).unwrap();
        self.player = Some(player);
        Ok(())
    }

    fn build(self) -> Player {
        self.player.unwrap()
    }
}

impl From<PromptError> for PlayerBuildError {
    fn from(value: PromptError) -> Self {
        Self::new(format!("Prompt Error:\n{value}"))
    }
}
