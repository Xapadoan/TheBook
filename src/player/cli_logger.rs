use std::error::Error;
use std::io;

use uuid::Uuid;

use crate::name::HasName;
use crate::repository::main::Repository;
use crate::repository::main::UniqueEntity;
use crate::warrior::Warrior;

use super::main::{Player, WarriorsManager};
use super::main::PlayerBuildFinalStep;
use super::main::PlayerBuildStepDisplayName;
use super::main::{PlayerBuildStepUserName, PlayerBuilder};

pub struct CliPlayerLogger<R: Repository<Player>> {
    repo: R,
    player: Option<Player>,
}

impl<R: Repository<Player>> CliPlayerLogger<R> {
    pub fn new(repo: R) -> Self {
        Self {
            repo,
            player: None,
        }
    }
}

impl<R: Repository<Player>> PlayerBuilder for CliPlayerLogger<R> {
    fn get_username(&mut self) -> Result<PlayerBuildStepUserName, Box<dyn Error>> {
        println!("Welcome back, enter your uuid:");
        let mut str = String::new();
        io::stdin().read_line(&mut str)?;
        let uuid = Uuid::parse_str(str.trim())?;
        let player = self.repo.get_by_uuid(&uuid)?;
        let username = player.username().clone();
        self.player = Some(player);
        Ok(PlayerBuildStepUserName::new(username))
    }

    fn get_display_name(&mut self, previous_step: PlayerBuildStepUserName) -> Result<PlayerBuildStepDisplayName, Box<dyn Error>> {
        let display_name = self.player.as_ref().unwrap().name().clone();
        Ok(PlayerBuildStepDisplayName::new(display_name, previous_step))
    }

    fn get_warriors(&mut self, previous_step: PlayerBuildStepDisplayName) -> Result<super::main::PlayerBuildFinalStep, Box<dyn Error>> {
        let mut player = self.player.take().unwrap();
        let mut warriors: Vec<Warrior> = vec![];
        while player.warriors().len() > 0 {
            let uuid = player.warriors()[0].uuid().clone();
            warriors.push(player.take_warrior(&uuid).unwrap())
        }
        Ok(PlayerBuildFinalStep::new(warriors, previous_step))
    }
}
