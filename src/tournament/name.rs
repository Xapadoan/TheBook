use rand::Rng;
use crate::{name::Name, random_dictionary::RandomDictionary};

pub struct TournamentNameDictionary {}

impl TournamentNameDictionary {
    pub fn new() -> Self {
        Self {}
    }
}

impl RandomDictionary<Name> for TournamentNameDictionary {
    fn get_random_item(&self) -> Name {
        Name::from(POSSIBLE_NAMES[rand::thread_rng().gen_range(0..POSSIBLE_NAMES_COUNT)])
    }
}

const POSSIBLE_NAMES_COUNT: usize = 2;
const POSSIBLE_NAMES: [&str; POSSIBLE_NAMES_COUNT] = [
    "Blood sausages tournament",
    "Beer fest of Mliuej",
];
