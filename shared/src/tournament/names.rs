use rand::Rng;

use crate::random::RandomDictionary;

pub struct TournamentNameDictionary {}

impl RandomDictionary<&'static str> for TournamentNameDictionary {
    fn random_item() -> &'static str {
        POSSIBLE_NAMES[rand::thread_rng().gen_range(0..POSSIBLE_NAMES_COUNT)]
    }
}

const POSSIBLE_NAMES_COUNT: usize = 27;
const POSSIBLE_NAMES: [&str; POSSIBLE_NAMES_COUNT] = [
    "Blood sausages tournament",
    "Beer fest",
    "Farmer Frenzy",
    "Farmer Showdown",
    "Crop Circle Challenge",
    "Seed Sowing Spectacle",
    "Harvest Hoedown",
    "Barnyard Bash",
    "Farmer Olympics",
    "Gardening Gala",
    "Haystack Hootenanny",
    "Livestock Tournament",
    "Rural Rumble",
    "Quack-off",
    "Farmer Fiesta",
    "Agrarian Adventure",
    "Jamboree of Farmers",
    "Meadows Mayhem",
    "Countryside Challenge",
    "Farm Fest Frenzy",
    "Rural Rumble",
    "Rustic Revelry",
    "Pastoral Playoffs",
    "Homestead Hootenanny",
    "Fields of Fun",
    "Country Carnival",
    "Backwoods Bonanza"
];
