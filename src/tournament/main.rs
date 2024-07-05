use rand::Rng;
use std::error::Error;
use std::fmt::{Debug, Display};

use crate::gen_random::GenRandom;
use crate::name::{HasName, Name};
use crate::warrior::weapon::{GiveWeapon, Weapon};
use crate::warrior::{self, Warrior};

use super::fight::{Fight, FightResult};

#[derive(Debug)]
enum TournamentErrorKind {
    AddContestantFailed(&'static str),
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

pub struct Tournament<'t> {
    name: Name,
    max_contestants: usize,
    contestants: Vec<&'t mut Warrior>,
    fights: Vec<Fight<'t>>,
    round_results: Vec<Vec<FightResult<'t>>>,
}

impl<'t> Tournament<'t> {
    pub fn new(name: &str, max_contestants: usize) -> Self {
        Self {
            name: Name::from(name),
            max_contestants,
            contestants: vec![],
            fights: vec![],
            round_results: vec![],
        }
    }

    pub fn add_contestant(&mut self, contestant: &'t mut Warrior) -> Result<(), Box<dyn Error>> {
        if self.contestants.len() + 1 > self.max_contestants {
            return Err(Box::new(
                TournamentError::new(
                    TournamentErrorKind::AddContestantFailed("Tournament will not allow more contestants")
                )
            ))
        }
        self.contestants.push(contestant);
        Ok(())
    }

    fn gen_random_fights(&mut self) {
        let nb_fights = self.contestants.len() / 2;
        dbg!(&nb_fights);

        let mut fights: Vec<Fight> = Vec::new();
        let mut i = 0;
        while i < nb_fights {
            let random_index = rand::thread_rng().gen_range(0..self.contestants.len());
            let blue_corner = self.contestants.swap_remove(random_index);
            let random_index = rand::thread_rng().gen_range(0..self.contestants.len());
            let red_corner = self.contestants.swap_remove(random_index);
            let fight = Fight::new(blue_corner, red_corner);
            // let fight = self.random_fight();
            let (fighter1, fighter2) = fight.fighters();
            println!(
                "For the {}nth fight, {} will oppose {}",
                i + 1,
                fighter1,
                fighter2
            );
            fights.push(fight);
            i += 1;
        }

        fights.reverse();
        self.fights = fights
    }

    fn fight_round(&mut self, round: usize) {
        let mut results: Vec<FightResult> = vec![];
        // self.gen_random_fights();
        self.round_results.push(vec![]);
        // while self.fights.len() > 0 {
        for fight in &mut self.fights {
            // let mut fight = self.fights.pop().unwrap();
            // results.push(fight.auto());
            let mut result = fight.auto();
            results.push(result);
            // println!("{}", fight_result.end_reason());
            // match fight_result.winner() {
            //     Some(warrior) => winners.push(warrior),
            //     None => {}
            // }
        }
        // self.round_results[round] = results;
    }

    // fn end_round(&mut self, round_results: Vec<FightResult>) {
    //     for mut result in round_results {
    //         self.
    //     }
    // }

    pub fn setup_next_round(&mut self, round: usize) {
        let mut winners: Vec<&mut Warrior> = vec![];
        for mut result in self.round_results[round].iter_mut() {
            let w = result.winner();
            self.contestants.push(w.unwrap());
        }
    }

    pub fn auto(&mut self) {
        let mut round = 0;
        let mut len = self.contestants.len();
        // while len > 1 {
            self.gen_random_fights();
            self.fight_round(round);
            self.setup_next_round(round);
            len = self.contestants.len();
            round += 1;
        // }
    }

    // }

    // fn random_fight(&mut self) -> Fight {
    //     let random_index = rand::thread_rng().gen_range(0..self.contestants.len());
    //     let blue_corner = self.contestants.swap_remove(random_index);
    //     let random_index = rand::thread_rng().gen_range(0..self.contestants.len());
    //     let red_corner = self.contestants.swap_remove(random_index);
    //     Fight::new(blue_corner, red_corner)
    // }
}

impl<'t> HasName for Tournament<'t> {
    fn name(&self) -> &Name {
        &self.name
    }
}
