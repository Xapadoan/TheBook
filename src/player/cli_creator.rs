use std::{error::Error, io};

use crate::{gen_random::GenRandom, warrior::Warrior};

use super::main::{PlayerBuildFinalStep, PlayerBuildStepDisplayName, PlayerBuildStepUserName, PlayerBuilder};

pub struct CliPlayerCreator {}

impl CliPlayerCreator {
    pub fn new() -> Self { Self {} }
}

impl PlayerBuilder for CliPlayerCreator {
    fn get_username(&self) -> Result<PlayerBuildStepUserName, Box<dyn Error>> {
        println!("Choose a username: ");
        let mut username = String::new();
        io::stdin().read_line(&mut username)?;
        Ok(PlayerBuildStepUserName::new(username.trim().to_string()))
    }

    fn get_display_name(&self, previous_step: PlayerBuildStepUserName) -> Result<PlayerBuildStepDisplayName, Box<dyn Error>> {
        println!("Choose a display name:");
        let mut display_name = String::new();
        io::stdin().read_line(&mut display_name)?;
        Ok(PlayerBuildStepDisplayName::new(display_name.trim().to_string(), previous_step))
    }

    fn get_warriors(&self, previous_step: PlayerBuildStepDisplayName) -> Result<super::main::PlayerBuildFinalStep, Box<dyn Error>> {
        let mut i = 0;
        let mut warriors = Vec::new();
        while i < 8 {
            let warrior = Warrior::gen_random();
            warriors.push(warrior);
            i += 1;
        }
        Ok(PlayerBuildFinalStep::new(warriors, previous_step))
    }
}