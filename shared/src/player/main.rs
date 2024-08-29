use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::unique_entity::UniqueEntity;
use crate::warrior::{MutableWarriorCollection, Warrior, WarriorCollection};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    uuid: Uuid,
    username: String,
    display_name: String,
    warriors: Vec<Warrior>,
}

impl Player {
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    // server only
    pub fn new(uuid: Uuid, username: String, display_name: String, warriors: Vec<Warrior>) -> Self {
        Self {
            uuid,
            username,
            display_name,
            warriors,
        }
    }
}

impl UniqueEntity for Player {
    fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}

impl WarriorCollection for Player {
    fn warriors(&self) -> &Vec<Warrior> {
        &self.warriors
    }
}

impl MutableWarriorCollection for Player {
    fn warriors_mut(&mut self) -> &mut Vec<Warrior> {
        &mut self.warriors
    }
}