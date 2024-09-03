use std::{collections::HashMap, error::Error};
use std::fmt::Display;

use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::inventory::MutableItems;
use crate::{inventory::Inventory, name::Name, random::{Random, RandomDictionary}, unique_entity::UniqueEntity};

use super::{contestant::TournamentContestant, TournamentNameDictionary};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tournament {
    uuid: Uuid,
    name: String,
    max_contestants: usize,
    // contestants_ids: Vec<Uuid>,
    contestants: HashMap<Uuid, Vec<Uuid>>,
    dropped_items: HashMap<Uuid, Inventory>,
}

impl Tournament {
    // server only
    pub fn add_contestant(&mut self, player_uuid: &Uuid, warrior: &dyn TournamentContestant) -> Result<(), TournamentError> {
        if self.number_of_contestants() + 1 > self.max_contestants {
            return Err(TournamentError::new(String::from("Tournament will not allow more contestants")));
        }
        if let Some(player_contestants) = self.contestants.get_mut(player_uuid) {
            player_contestants.push(warrior.uuid().clone());
        } else {
            self.contestants.insert(player_uuid.clone(), vec![warrior.uuid().clone()]);
        }
        Ok(())
    }

    pub fn max_contestants(&self) -> usize {
        self.max_contestants
    }

    // server only
    fn new(name: String, max_contestants: usize) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            max_contestants,
            // contestants_ids: vec![],
            contestants: HashMap::new(),
            dropped_items: HashMap::new(),
        }
    }

    pub fn number_of_contestants(&self) -> usize {
        let mut count = 0;
        for player_contestants in self.contestants.values() {
            count += player_contestants.len();
        }
        count
    }

    pub fn number_of_rounds(&self) -> usize {
        self.number_of_contestants() / 2
    }

    pub fn is_full(&self) -> bool {
        self.number_of_contestants() >= self.max_contestants
    }

    pub fn contestants(&self) -> &HashMap<Uuid, Vec<Uuid>> {
        &self.contestants
    }

    // server_only
    pub fn contestants_ids(&self) -> Vec<Uuid> {
        let mut all_contestants = vec![];
        for player_contestants in self.contestants.values() {
            all_contestants = [
                all_contestants,
                player_contestants.to_vec(),
            ].concat()
        }
        all_contestants
    }

    pub fn dropped_items(&self) -> &HashMap<Uuid, Inventory> {
        &self.dropped_items
    }

    // server only
    pub fn add_dropped_items(
        &mut self,
        warrior_uuid: &Uuid,
        mut dropped_items: Inventory,
    ) {
        if let Some(already_dropped_items) = self.dropped_items.get_mut(warrior_uuid) {
            for (_, item) in dropped_items.items_mut().drain() {
                already_dropped_items.add_item(item);
            }
        } else {
            self.dropped_items.insert(warrior_uuid.clone(), dropped_items);
        }
    }

    //server only
    pub fn take_dropped_items(
        &mut self,
        warrior_uuid: &Uuid,
    ) -> Option<Inventory> {
        self.dropped_items.remove(warrior_uuid)
    }
}

impl Name for Tournament {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Random for Tournament {
    fn random() -> Self {
        let pow = rand::thread_rng().gen_range(1..3);
        let mut max_contestants = 2;
        let mut i = 0;
        while i < pow {
            max_contestants *= 2;
            i += 1;
        }
        Self::new(
            String::from(TournamentNameDictionary::random_item()),
            max_contestants
        )
    }
}

impl UniqueEntity for Tournament {
    fn uuid<'a>(&'a self) -> &'a Uuid {
        &self.uuid
    }
}

#[derive(Debug)]
pub struct TournamentError {
    message: String,
}

impl TournamentError {
    pub fn new(message: String) -> Self {
        Self { message: format!("Tournament Error:\n{message}") }
    }
}

impl Display for TournamentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for TournamentError {}

#[cfg(test)]
mod test {
    use crate::warrior::Warrior;

    use super::*;

    #[test]
    fn should_not_accept_more_than_max_contestants() {
        let mut tournament = Tournament {
            uuid: Uuid::new_v4(),
            name: String::from(TournamentNameDictionary::random_item()),
            max_contestants: 1,
            contestants: HashMap::new(),
            dropped_items: HashMap::new(),
        };
        let player_uuid = Uuid::new_v4();
        let warrior = Warrior::random();
        let result = tournament.add_contestant(&player_uuid, &warrior);
        assert!(!result.is_ok())
    }

    #[test]
    fn should_add_warrior_uuid_when_accepting() {
        let mut tournament = Tournament {
            uuid: Uuid::new_v4(),
            name: String::from(TournamentNameDictionary::random_item()),
            max_contestants: 2,
            contestants: HashMap::new(),
            dropped_items: HashMap::new(),
        };
        let mut expected_uuids: Vec<Uuid> = vec![];
        let player_uuid = Uuid::new_v4();
        let warrior = Warrior::random();
        expected_uuids.push(warrior.uuid().clone());
        let result = tournament.add_contestant(&player_uuid, &warrior);
        assert!(result.is_ok());
        assert_eq!(tournament.contestants_ids(), expected_uuids);

        let warrior = Warrior::random();
        expected_uuids.push(warrior.uuid().clone());
        let result = tournament.add_contestant(&player_uuid, &warrior);
        assert!(result.is_ok());
        assert_eq!(tournament.contestants_ids(), expected_uuids);
    }
}
