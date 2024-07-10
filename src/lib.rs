mod dice;
mod modifiers;
mod equipment;
mod tournament {
    pub mod main;
    mod fight;
    mod name;
}
mod warrior;
mod virtual_timer;
mod name;
mod gen_random;
pub mod player {
    pub mod main;
    pub mod cli_creator;
    pub mod cli_logger;
    pub mod repository;
}
pub mod repository {
    pub mod main;
    pub mod file_repository;
}
pub mod random_dictionary;

use std::error::Error;
use std::io;
use std::path::PathBuf;

use name::HasName;
use player::cli_creator::CliPlayerCreator;
use player::cli_logger::CliPlayerLogger;
use player::main::{Player, WarriorsManager};
use gen_random::GenRandom;
use player::repository::PlayerRepository;
use repository::file_repository::FileRepository;
use repository::main::{Repository, UniqueEntity};
use warrior::Warrior;
use warrior::weapon::{Weapon, GiveWeapon};
use warrior::protection::{Protection, ProtectionKind, WearProtection};
use warrior::body::body_part::BodyPartKind;
use warrior::body::body_side::BodySide;

impl Warrior {
    fn wear_random_protection(&mut self, protection: Protection) {
        match protection.kind() {
            ProtectionKind::Armlet => self.wear_protection(protection, BodyPartKind::Arm(BodySide::gen_random())),
            ProtectionKind::Boot => self.wear_protection(protection, BodyPartKind::Foot(BodySide::gen_random())),
            ProtectionKind::ChainMail => self.wear_protection(protection, BodyPartKind::Torso),
            ProtectionKind::Gambeson => self.wear_protection(protection, BodyPartKind::Torso),
            ProtectionKind::Gauntlet => self.wear_protection(protection, BodyPartKind::Hand(BodySide::gen_random())),
            ProtectionKind::Greave => self.wear_protection(protection, BodyPartKind::Leg(BodySide::gen_random())),
            ProtectionKind::Helm => self.wear_protection(protection, BodyPartKind::Head),
            ProtectionKind::Jacket => self.wear_protection(protection, BodyPartKind::Torso),
            ProtectionKind::Plastron => self.wear_protection(protection, BodyPartKind::Torso),
        };
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut player = welcome_player()?;
    dbg!(&player);
    // play(&mut player)?;
    Ok(())
}

fn welcome_player() -> Result<Player, Box<dyn Error>> {
    let repo = PlayerRepository::build()?;
    println!("Do you already have an account ?\n (Y / N)");
    let mut user_response = String::new();
    io::stdin().read_line(&mut user_response)?;
    let player = if user_response.trim().to_lowercase() == "y" {
        let mut builder = CliPlayerLogger::build()?;
        Player::build(&mut builder)?
    } else  {
        let mut builder = CliPlayerCreator::new();
        let tmp = Player::build(&mut builder)?;
        repo.create(&tmp)?;
        tmp
    };
    Ok(player)
}

// fn play(player: &mut Player) -> Result<(), Box<dyn Error>> {
//     let mut tournament = Tournament::gen_random();
//     println!("A tournament, the {} will start soon, do you want to send warriors ? (Y/N)", tournament.name());
//     let mut user_response = String::new();
//     io::stdin().read_line(&mut user_response)?;
//     if user_response.trim().to_lowercase() != "y" {
//         println!("Ok Bye !");
//         return Ok(());
//     }
//     println!("Select a warrior:");
//     let mut i = 0;
//     let warriors = player.warriors_mut();
//     while i < warriors.len() {
//         println!("{}. {}", i + 1, warriors[i].name());
//         i += 1;
//     }
//     user_response.clear();
//     io::stdin().read_line(&mut user_response)?;
//     let mut index: usize = user_response.trim().parse()?;
//     index -= 1;
//     let warrior = &mut warriors[index];
//     tournament.add_contestant(warrior)?;

//     let mut w = Warrior::gen_random();
//     let we = Weapon::gen_random();
//     w.give_weapon(we);
//     tournament.add_contestant(&mut w)?;

//     let mut w = Warrior::gen_random();
//     let we = Weapon::gen_random();
//     w.give_weapon(we);
//     tournament.add_contestant(&mut w)?;

//     let mut w = Warrior::gen_random();
//     let we = Weapon::gen_random();
//     w.give_weapon(we);
//     tournament.add_contestant(&mut w)?;

//     let mut w = Warrior::gen_random();
//     let we = Weapon::gen_random();
//     w.give_weapon(we);
//     tournament.add_contestant(&mut w)?;

//     let mut w = Warrior::gen_random();
//     let we = Weapon::gen_random();
//     w.give_weapon(we);
//     tournament.add_contestant(&mut w)?;

//     let mut w = Warrior::gen_random();
//     let we = Weapon::gen_random();
//     w.give_weapon(we);
//     tournament.add_contestant(&mut w)?;

//     let mut w = Warrior::gen_random();
//     let we = Weapon::gen_random();
//     w.give_weapon(we);
//     tournament.add_contestant(&mut w)?;

//     tournament.auto();
//     let repo = FileRepository::build(PathBuf::from("saves/players"))?;
//     dbg!(warrior);
//     repo.update(player.uuid(), player)?;
//     Ok(())
// }
