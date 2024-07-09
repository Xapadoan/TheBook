use rand::Rng;
use crate::{name::Name, random_dictionary::RandomDictionary};

pub struct WarriorNameDictionary {}

impl WarriorNameDictionary {
    pub fn new() -> Self {
        Self {}
    }
}

impl RandomDictionary<Name> for WarriorNameDictionary {
    fn get_random_item(&self) -> Name {
        Name::from(POSSIBLE_NAMES[rand::thread_rng().gen_range(0..POSSIBLE_NAMES_COUNT)])
    }
}

const POSSIBLE_NAMES_COUNT: usize = 82;
const POSSIBLE_NAMES: [&str; POSSIBLE_NAMES_COUNT] = [
    "Adeldryt",
    "Ald",
    "Anlett",
    "Ann'ne",
    "Arcen",
    "Beodino",
    "Beornba",
    "Brandret",
    "Brytchris",
    "Carbeorn",
    "Casead",
    "Ciarich",
    "Chei",
    "Cona",
    "Crowpa",
    "Cuthven",
    "Cuthwlad",
    "Dasnath",
    "Dicsephregin",
    "Dod",
    "Eadta",
    "Eadwighty",
    "Eter",
    "Finul",
    "Frithgifu",
    "Gardsam",
    "Garfastsam",
    "Gifulee",
    "Grimha",
    "Halaf",
    "Hasaroy",
    "Hawig",
    "Ha'wise",
    "Hildiswith",
    "Ja",
    "Jaria",
    "Kelay",
    "Kencar",
    "Kensa",
    "Lehtobel",
    "Leophu",
    "Liambando",
    "Liethel",
    "Macfast",
    "Masarma",
    "Maxda",
    "Mei",
    "Merythor",
    "Mesdryt",
    "Morbiff",
    "Naacar",
    "Nas",
    "Nethphie",
    "Niswith",
    "Nithu",
    "Paulorah",
    "Phieca",
    "Phietom",
    "Rahthaetom",
    "Redleof",
    "Regingarsa",
    "Reginvin",
    "Reteli",
    "Retjoan",
    "Riajo",
    "Rolham",
    "Roymuel",
    "Ryrolpa",
    "Sacla",
    "Saan",
    "Swithmuel",
    "Sylja",
    "Tendark",
    "Tho",
    "Timswith",
    "Tinebrasson",
    "Tolhelm",
    "Vellcy",
    "Vellra",
    "Venles",
    "Vingrin",
    "Wardevertine",
];
