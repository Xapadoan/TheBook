use crate::fight::Fight;
use crate::warrior::Warrior;
use rand::Rng;

pub struct Tournament {
    contestants: Vec<Warrior>,
}

impl Tournament {
    pub fn new(contestants: Vec<Warrior>) -> Tournament {
        println!("A tournament is about to begin");
        Tournament { contestants }
    }

    pub fn fight_round(&mut self, n: u8) {
        println!("=== Starting {}nth round ===", n);
        let nb_fights = self.contestants.len() / 2;

        let mut fights: Vec<Fight> = Vec::new();
        let mut i = 0;
        while i < nb_fights {
            let fight = self.random_fight();
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
        while fights.len() > 0 {
            let fight = fights.pop().unwrap();
            let fight_result = fight.auto();
            println!("{}", fight_result.end_reason());
            match fight_result.winner() {
                Some(warrior) => self.contestants.push(warrior),
                None => {}
            }
        }
    }

    fn random_fight(&mut self) -> Fight {
        let mut random_index = rand::thread_rng().gen_range(0..self.contestants.len());
        let blue_corner = self.contestants.swap_remove(random_index);
        random_index = rand::thread_rng().gen_range(0..self.contestants.len());
        let red_corner = self.contestants.swap_remove(random_index);
        Fight::new(blue_corner, red_corner)
    }
}
