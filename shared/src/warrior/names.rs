use rand::Rng;
use crate::random::RandomDictionary;

pub struct WarriorNameDictionary {}

impl RandomDictionary<&'static str> for WarriorNameDictionary {
    fn random_item() -> &'static str {
        POSSIBLE_NAMES[rand::thread_rng().gen_range(0..POSSIBLE_NAMES_COUNT)]
    }
}

const POSSIBLE_NAMES_COUNT: usize = 198;
const POSSIBLE_NAMES: [&'static str; POSSIBLE_NAMES_COUNT] = [
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
    "Jhimmri",
    "Valonov",
    "Lirandrin",
    "Kedov",
    "Kedrina",
    "Aganov",
    "Orlardrick",
    "Walley",
    "Mavina",
    "Tenescohan",
    "Ehvela",
    "Kinakirn",
    "Maliri",
    "Mortesugh",
    "Orrestag",
    "Zorov",
    "Strynira",
    "Lanov",
    "Dorana",
    "Kanouez",
    "Gordrik",
    "Gotov",
    "Ekavstin",
    "Larbø",
    "Jhesta",
    "Veyevrozov",
    "Rostina",
    "Rolleinov",
    "Selmrik",
    "Carkleyev",
    "Klubev",
    "Belvin",
    "Matvellber",
    "Goara",
    "Hengstessy",
    "Orgessy",
    "Ehvelira",
    "Pflarrin",
    "Medrina",
    "Essonnov",
    "Delmdiaria",
    "Yadonov",
    "Orrna",
    "Egolz",
    "Delmris",
    "Laghan",
    "Ehvelstin",
    "Xansensev",
    "Morda",
    "Ullerömoto",
    "Lysandria",
    "Yamoton",
    "Lisrarin",
    "Enkov",
    "Ordra",
    "Ahlstov",
    "Liestrik",
    "Mikkearov",
    "Goesra",
    "Covanner",
    "Ezeen",
    "Ahlergaden",
    "Delara",
    "Esson",
    "Illack",
    "Shitzhanov",
    "Lirstrenen",
    "Vauergsto",
    "Haellara",
    "Okaarbø",
    "Alara",
    "Miyakatsen",
    "Lirelirina",
    "Railbruck",
    "Calliren",
    "Albev",
    "Sandrik",
    "Chneyev",
    "Faelora",
    "Lagano",
    "Delmdia",
    "Wanssen",
    "Zinaranov",
    "Malori",
    "Ularedan",
    "Ehressa",
    "Thagnez",
    "Faelmrin",
    "Eisen",
    "Eliria",
    "Gadov",
    "Jhesvin",
    "Joharbø",
    "Ryamoto",
    "Belstreena",
    "Zyugar",
    "Dorina",
    "Gotonney",
    "Alisina",
    "Shimnyev",
    "Carvina",
    "Miyako",
    "Faela",
    "Zohannerg",
    "Carin",
    "Chrisev",
    "Dorla",
    "Lundgredo",
    "Malari",
    "Dronov",
    "Jhessa",
    "Alonov",
    "Jhesvina",
    "Zimenkov",
    "Telvin",
    "Yamototov",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_must_be_unique() {
        for i in 0..POSSIBLE_NAMES_COUNT {
            for j in (i + 1)..POSSIBLE_NAMES_COUNT {
                assert!(
                    POSSIBLE_NAMES[i] != POSSIBLE_NAMES[j],
                    "{} is not unique",
                    POSSIBLE_NAMES[i],
                )
            }
            
        }
    }
}
