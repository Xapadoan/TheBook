use std::path::PathBuf;

use crate::client::prompt::prompt;
use crate::repository::{FileRepository, Repository};
use shared::equipment::weapon::{OptionalMutableWeapon, Weapon};
use shared::player::{Player, PlayerBuildError, PlayerBuilder};
use shared::random::Random;
use shared::warrior::Warrior;
use uuid::Uuid;

use super::prompt::PromptError;

pub struct PlayerCreator {
    username: Option<String>,
    display_name: Option<String>,
    warriors: Vec<Warrior>,
}

impl PlayerCreator {
    pub fn new() -> Self {
        Self { username: None, display_name: None, warriors: vec![] }
    }
}

impl PlayerBuilder for PlayerCreator {
    fn get_username(&mut self) -> Result<(), PlayerBuildError> {
        let username = prompt("Choose a username:")?;
        self.username = Some(username);
        Ok(())
    }

    fn get_display_name(&mut self) -> Result<(), PlayerBuildError> {
        let display_name = prompt("Choose a display name:")?;
        self.display_name = Some(display_name);
        Ok(())
    }

    fn get_warriors(&mut self) -> Result<(), PlayerBuildError> {
        let mut i = 0;
        let repo = FileRepository::build(PathBuf::from("saves/warriors"))?;
        while i < 8 {
            let mut warrior = Warrior::random();
            let weapon = Weapon::random();
            warrior.replace_weapon(weapon);
            repo.create(&warrior)?;
            self.warriors.push(warrior);
            i += 1;
        }
        Ok(())
    }

    fn build(self) -> Player {
        Player::new(
            Uuid::new_v4(),
            self.username.unwrap(),
            self.display_name.unwrap(),
            self.warriors
        )
    }
}

impl From<PromptError> for PlayerBuildError {
    fn from(value: PromptError) -> Self {
        Self::new(format!("Prompt Error:\n{value}"))
    }
}
