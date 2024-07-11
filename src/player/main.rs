use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::gen_random::GenRandom;
use crate::name::{HasName, Name};
use crate::repository::file_repository::FileRepository;
use crate::repository::main::{Repository, UniqueEntity};
use crate::warrior::{IsDead, Warrior};

use super::repository::PlayerRepository;

pub trait WarriorsManager {
    fn warriors<'a>(&'a self) -> &'a Vec<Warrior>;
    fn warriors_mut<'a>(&'a mut self) -> &'a mut Vec<Warrior>;
    fn give_warrior(&mut self, warrior: Warrior);
    fn take_warrior(&mut self, uuid: &Uuid) -> Option<Warrior>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    uuid: Uuid,
    username: Name,
    display_name: Name,
    warriors: Vec<Warrior>,
}

impl Player {
    pub fn username<'a>(&'a self) -> &'a Name {
        &self.username
    }

    pub fn replace_dead_warriors(&mut self) -> Result<(), Box<dyn Error>> {
        let mut dead_warriors_uuids: Vec<Uuid> = vec![];
        for warrior in self.warriors() {
            if warrior.is_dead() {
                dead_warriors_uuids.push(warrior.uuid().clone());
            }
        }
        for uuid in dead_warriors_uuids {
            if let Some(w) = self.take_warrior(&uuid) {
                println!("{} died during the last tournament", w.name());
                let repo: FileRepository<Warrior> = FileRepository::build(PathBuf::from("saves/warriors"))?;
                repo.delete(w.uuid())?;
            }
            let w = Warrior::gen_random();
            println!("{} will join your team", w.name());
            self.give_warrior(w);
        }
        let repo = PlayerRepository::build()?;
        repo.update(self.uuid(), self)?;
        Ok(())
    }
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

    fn warriors_mut<'a>(&'a mut self) -> &'a mut Vec<Warrior> {
        self.warriors.as_mut()
    }

    fn give_warrior(&mut self, warrior: Warrior) {
        self.warriors.push(warrior)
    }

    fn take_warrior(&mut self, uuid: &Uuid) -> Option<Warrior> {
        let position = self.warriors.iter().position(
            |w|
            {
                w.uuid() == uuid
            }
        );
        match position {
            Some(index) => Some(self.warriors.swap_remove(index)),
            None => None
        }
    }
}

pub struct PlayerBuildStepUserName {
    username: Name,
}

impl PlayerBuildStepUserName {
    pub fn new(username: Name) -> Self {
        Self { username }
    }

    pub fn username(&self) -> &str {
        &self.username
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
    fn get_username(&mut self) -> Result<PlayerBuildStepUserName, PlayerBuildError>;
    fn get_display_name(&mut self, previous_step: PlayerBuildStepUserName) -> Result<PlayerBuildStepDisplayName, PlayerBuildError>;
    fn get_warriors(&mut self, previous_step: PlayerBuildStepDisplayName) -> Result<PlayerBuildFinalStep, PlayerBuildError>;
}

impl Player {
    fn new(player_build: PlayerBuildFinalStep) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            username: player_build.username,
            display_name: player_build.display_name,
            warriors: player_build.warriors,
        }
    }

    pub fn build(builder: &mut impl PlayerBuilder) -> Result<Player, PlayerBuildError> {
        let username_step = builder.get_username()?;
        let display_name_step = builder.get_display_name(username_step)?;
        let final_step = builder.get_warriors(display_name_step)?;
        Ok(Self::new(final_step))
    }
}

impl UniqueEntity for Player {
    fn uuid<'a>(&'a self) -> &'a Uuid {
        &self.uuid
    }
}

#[derive(Debug)]
pub struct PlayerBuildError {
    message: String,
}

impl PlayerBuildError {
    pub fn new(message: String) -> Self {
        Self {
            message: format!("PlayerBuildError:\n{message}")
        }
    }
}

impl Display for PlayerBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PlayerBuildError {}
