use std::path::PathBuf;

use shared::auth::Session;
use shared::equipment::weapon::{OptionalMutableWeapon, Weapon};
use shared::inventory::Inventory;
use shared::player::{Player, PlayerBuildError, PlayerBuilder};
use shared::random::Random;
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;
use uuid::Uuid;

use crate::repository::{FileRepository, PlayerRepository, Repository};

use super::{AuthAPIError, SessionManager};

pub fn signup(username: String, display_name: String) -> Result<Session, AuthAPIError> {
    let mut signup = SignUp::new(username, display_name);
    if let Err(_) = signup.build_warriors() {
        panic!("SignUp.build_warriors() should never return error")
    }
    let player = signup.build();
    let repo = PlayerRepository::build()?;
    repo.create(&player)?;
    let session_manager = SessionManager::build()?;
    let session = session_manager.create_session(player.uuid())?;
    Ok(session)
}

struct SignUp {
    username: String,
    display_name: String,
    warriors: Vec<Warrior>,
}

impl SignUp {
    pub fn new(username: String, display_name: String) -> Self {
        Self {
            username,
            display_name,
            warriors: vec![],
        }
    }
}

impl PlayerBuilder for SignUp {
    fn build_username(&mut self) -> Result<(), PlayerBuildError> {
        Ok(())
    }
    fn build_display_name(&mut self) -> Result<(), PlayerBuildError> {
        Ok(())
    }
    fn build_warriors(&mut self) -> Result<(), PlayerBuildError> {
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
    fn build_inventory(&mut self) -> Result<(), PlayerBuildError> {
        Ok(())
    }
    fn build(self) -> Player {
        Player::new(
            Uuid::new_v4(),
            self.username,
            self.display_name,
            self.warriors,
            Inventory::new(),
        )
    }
}
