use rand::Rng;
use uuid::Uuid;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::path::PathBuf;

use crate::gen_random::GenRandom;
use crate::name::{HasName, Name};
use crate::random_dictionary::RandomDictionary;
use crate::repository::file_repository::FileRepository;
use crate::repository::main::{Repository, UniqueEntity};
use crate::tournament::fight::FightResultKind;
use crate::warrior::Warrior;

use super::fight::Fight;
use super::name::TournamentNameDictionary;

#[derive(Debug)]
enum TournamentErrorKind {
    AddContestantFailed(&'static str),
    WarriorNotFound(String),
}

#[derive(Debug)]
struct TournamentError {
    kind: TournamentErrorKind,
}

impl TournamentError {
    fn new(kind: TournamentErrorKind) -> Self {
        Self { kind }
    } 
}

impl Display for TournamentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Error for TournamentError {}

pub struct Tournament {
    name: Name,
    max_contestants: usize,
    contestants_ids: Vec<Uuid>,
}

impl Tournament {
    pub fn new(name: Name, max_contestants: usize) -> Self {
        Self {
            name,
            max_contestants,
            contestants_ids: vec![],
        }
    }

    pub fn add_contestant(&mut self, warrior: &Warrior) -> Result<(), Box<dyn Error>> {
        if self.contestants_ids.len() + 1 > self.max_contestants {
            return Err(Box::new(
                TournamentError::new(
                    TournamentErrorKind::AddContestantFailed("Tournament will not allow more contestants")
                )
            ))
        }
        self.contestants_ids.push(warrior.uuid().clone());
        Ok(())
    }

    fn gen_random_pairs(&mut self) -> Vec<(Uuid, Uuid)> {
        let mut contestants_count = self.contestants_ids.len();
        dbg!(contestants_count);
        let nb_fights = contestants_count / 2;
        dbg!(&nb_fights);

        let mut pairs: Vec<(Uuid, Uuid)> = vec![];
        let mut i = 0;
        while i < nb_fights {
            let random_index = rand::thread_rng().gen_range(0..contestants_count);
            let blue_corner = self.contestants_ids.swap_remove(random_index);
            contestants_count -= 1;
            let random_index = rand::thread_rng().gen_range(0..contestants_count);
            let red_corner = self.contestants_ids.swap_remove(random_index);
            contestants_count -= 1;
            pairs.push((red_corner, blue_corner));
            i += 1;
        }
        pairs
    }

    fn get_warriors<T: Repository<Warrior>>(
        repo: &T,
        uuid1: &Uuid,
        uuid2: &Uuid,
    ) -> Result<(Warrior, Warrior), Box<dyn Error>> {
        let w1 = repo.get_by_uuid(uuid1);
        if w1.is_err() {
            return Err(Box::new(TournamentError::new(
                TournamentErrorKind::WarriorNotFound(format!("Warrior not found (uuid: {uuid1})"))
            )));
        }
        let w2 = repo.get_by_uuid(uuid2);
        if w2.is_err() {
            return Err(Box::new(TournamentError::new(
                TournamentErrorKind::WarriorNotFound(format!("Warrior not found (uuid: {uuid2})"))
            )));
        }
        Ok((w1.ok().unwrap(), w2.ok().unwrap()))
    }

    pub fn auto(&mut self) -> Result<(), Box<dyn Error>> {
        let repo = FileRepository::build(PathBuf::from("saves/warriors"))?;
        let mut round = 0;
        dbg!(&self.contestants_ids);
        let mut len = self.contestants_ids.len();
        while len > 1 {
            println!("Start Round: {}", round + 1);
            let pairs = self.gen_random_pairs();
            for pair in pairs {
                dbg!(&pair);
                let (warrior1, warrior2) = Self::get_warriors(&repo, &pair.1, &pair.0)?;
                let result = Fight::new(warrior1, warrior2).auto();
                match result.kind() {
                    FightResultKind::Victory(fighters) => {
                        repo.update(fighters.winner().uuid(), fighters.winner())?;
                        repo.update(fighters.loser().uuid(), fighters.loser())?;
                        self.contestants_ids.push(fighters.winner().uuid().clone());
                    },
                    FightResultKind::Tie((warrior1, warrior2)) => {
                        repo.update(warrior1.uuid(), warrior1)?;
                        repo.update(warrior2.uuid(), warrior2)?;
                    }
                }
            }
            len = self.contestants_ids.len();
            round += 1;
        }
        Ok(())
    }
}

impl HasName for Tournament {
    fn name(&self) -> &Name {
        &self.name
    }
}

impl GenRandom for Tournament {
    fn gen_random() -> Self {
        let pow = rand::thread_rng().gen_range(1..10);
        let mut max_contestants = 2;
        let mut i = 0;
        while i < pow {
            max_contestants *= 2;
            i += 1;
        }
        Self::new(
            TournamentNameDictionary::new().get_random_item(),
            max_contestants
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_not_accept_more_than_max_contestants() {
        let mut tournament = Tournament {
            name: TournamentNameDictionary::new().get_random_item(),
            max_contestants: 1,
            contestants_ids: vec![Uuid::new_v4()],
        };
        let warrior = Warrior::gen_random();
        let result = tournament.add_contestant(&warrior);
        assert!(!result.is_ok())
    }

    #[test]
    fn should_add_warrior_uuid_when_accepting() {
        let mut tournament = Tournament {
            name: TournamentNameDictionary::new().get_random_item(),
            max_contestants: 2,
            contestants_ids: vec![],
        };
        let mut expected_uuids: Vec<Uuid> = vec![];
        let warrior = Warrior::gen_random();
        expected_uuids.push(warrior.uuid().clone());
        let result = tournament.add_contestant(&warrior);
        assert!(result.is_ok());
        assert_eq!(tournament.contestants_ids, expected_uuids);

        let warrior = Warrior::gen_random();
        expected_uuids.push(warrior.uuid().clone());
        let result = tournament.add_contestant(&warrior);
        assert!(result.is_ok());
        assert_eq!(tournament.contestants_ids, expected_uuids);
    }
}
