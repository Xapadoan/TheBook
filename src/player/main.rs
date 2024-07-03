use std::error::Error;

use crate::name::{HasName, Name};
use crate::warrior::Warrior;

trait WarriorsManager {
    fn warriors<'a>(&'a self) -> &'a Vec<Warrior>;
    fn add_warrior(&mut self, warrior: Warrior);
}

#[derive(Debug)]
pub struct Player {
    username: Name,
    display_name: Name,
    warriors: Vec<Warrior>,
}

impl HasName for Player {
    fn name<'a>(&'a self) -> &'a Name {
        &self.display_name
    }
}

impl WarriorsManager for Player {
    fn warriors<'a>(&'a self) -> &'a Vec<Warrior> {
        &self.warriors
    }

    fn add_warrior(&mut self, warrior: Warrior) {
        self.warriors.push(warrior)
    }
}


pub struct PlayerBuildStepUserName {
    username: Name,
}

impl PlayerBuildStepUserName {
    pub fn new(username: Name) -> Self {
        Self { username }
    }
}

pub struct PlayerBuildStepDisplayName {
    username: Name,
    display_name: Name,
}

impl PlayerBuildStepDisplayName {
    pub fn new(display_name: Name, previous_step: PlayerBuildStepUserName) -> Self {
        Self {
            username: previous_step.username,
            display_name
        }
    }
}

pub struct PlayerBuildFinalStep {
    username: Name,
    display_name: Name,
    warriors: Vec<Warrior>,
}

impl PlayerBuildFinalStep {
    pub fn new(warriors: Vec<Warrior>, previous_step: PlayerBuildStepDisplayName) -> Self {
        Self {
            username: previous_step.username,
            display_name: previous_step.display_name,
            warriors
        }
    }
}

pub trait PlayerBuilder {
    fn get_username(&self) -> Result<PlayerBuildStepUserName, Box<dyn Error>>;
    fn get_display_name(&self, previous_step: PlayerBuildStepUserName) -> Result<PlayerBuildStepDisplayName, Box<dyn Error>>;
    fn get_warriors(&self, previous_step: PlayerBuildStepDisplayName) -> Result<PlayerBuildFinalStep, Box<dyn Error>>;
}

impl Player {
    fn new(player_build: PlayerBuildFinalStep) -> Self {
        Self {
            username: player_build.username,
            display_name: player_build.display_name,
            warriors: player_build.warriors,
        }
    }

    pub fn build(builder: &impl PlayerBuilder) -> Result<Player, Box<dyn Error>> {
        let username_step = builder.get_username()?;
        let display_name_step = builder.get_display_name(username_step)?;
        let final_step = builder.get_warriors(display_name_step)?;
        Ok(Self::new(final_step))
    }
}
