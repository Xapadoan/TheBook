use std::io;
use std::path::PathBuf;

use crate::gen_random::GenRandom;
use crate::repository::file_repository::FileRepository;
use crate::repository::main::Repository;
use crate::warrior::weapon::{GiveWeapon, Weapon};
use crate::warrior::Warrior;
use crate::player::main::{PlayerBuildError, PlayerBuildFinalStep, PlayerBuildStepDisplayName, PlayerBuildStepUserName, PlayerBuilder};

pub struct PlayerCreator {}

impl PlayerCreator {
    pub fn new() -> Self { Self {} }
}

impl PlayerBuilder for PlayerCreator {
    fn get_username(&mut self) -> Result<PlayerBuildStepUserName, PlayerBuildError> {
        println!("Choose a username: ");
        let mut username = String::new();
        io::stdin().read_line(&mut username)?;
        Ok(PlayerBuildStepUserName::new(username.trim().to_string()))
    }

    fn get_display_name(&mut self, previous_step: PlayerBuildStepUserName) -> Result<PlayerBuildStepDisplayName, PlayerBuildError> {
        println!("Choose a display name:");
        let mut display_name = String::new();
        io::stdin().read_line(&mut display_name)?;
        Ok(PlayerBuildStepDisplayName::new(display_name.trim().to_string(), previous_step))
    }

    fn get_warriors(&mut self, previous_step: PlayerBuildStepDisplayName) -> Result<PlayerBuildFinalStep, PlayerBuildError> {
        let mut i = 0;
        let mut warriors = Vec::new();
        let repo = FileRepository::build(PathBuf::from("saves/warriors"))?;
        while i < 8 {
            let mut warrior = Warrior::gen_random();
            let weapon = Weapon::gen_random();
            warrior.give_weapon(weapon);
            repo.create(&warrior)?;
            warriors.push(warrior);
            i += 1;
        }
        Ok(PlayerBuildFinalStep::new(warriors, previous_step))
    }
}