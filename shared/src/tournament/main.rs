use std::error::Error;
use std::fmt::Display;

use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{name::Name, random::{Random, RandomDictionary}, unique_entity::UniqueEntity};

use super::{contestant::TournamentContestant, TournamentNameDictionary};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tournament {
    uuid: Uuid,
    name: String,
    max_contestants: usize,
    contestants_ids: Vec<Uuid>,
}

impl Tournament {
    // server only
    pub fn add_contestant(&mut self, warrior: &dyn TournamentContestant) -> Result<(), TournamentError> {
        if self.contestants_ids.len() + 1 > self.max_contestants {
            return Err(TournamentError::new(String::from("Tournament will not allow more contestants")));
        }
        self.contestants_ids.push(warrior.uuid().clone());
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
            contestants_ids: vec![],
        }
    }

    pub fn number_of_contestants(&self) -> usize {
        self.contestants_ids.len()
    }

    pub fn number_of_rounds(&self) -> usize {
        self.number_of_contestants() / 2
    }

    pub fn is_full(&self) -> bool {
        self.contestants_ids.len() >= self.max_contestants
    }

    // server only
    pub fn contestants_ids_mut(&mut self) -> &mut Vec<Uuid> {
        &mut self.contestants_ids
    }

    pub fn contestants_ids(&self) -> &Vec<Uuid> {
        &self.contestants_ids
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
            contestants_ids: vec![Uuid::new_v4()],
        };
        let warrior = Warrior::random();
        let result = tournament.add_contestant(&warrior);
        assert!(!result.is_ok())
    }

    #[test]
    fn should_add_warrior_uuid_when_accepting() {
        let mut tournament = Tournament {
            uuid: Uuid::new_v4(),
            name: String::from(TournamentNameDictionary::random_item()),
            max_contestants: 2,
            contestants_ids: vec![],
        };
        let mut expected_uuids: Vec<Uuid> = vec![];
        let warrior = Warrior::random();
        expected_uuids.push(warrior.uuid().clone());
        let result = tournament.add_contestant(&warrior);
        assert!(result.is_ok());
        assert_eq!(tournament.contestants_ids, expected_uuids);

        let warrior = Warrior::random();
        expected_uuids.push(warrior.uuid().clone());
        let result = tournament.add_contestant(&warrior);
        assert!(result.is_ok());
        assert_eq!(tournament.contestants_ids, expected_uuids);
    }
}
