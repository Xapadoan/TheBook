mod dice;
mod fight;
mod modifiers;
mod equipment;
mod tournament;
mod warrior;
mod virtual_timer;
mod name;
mod gen_random;
mod save {
    pub mod save_manager;
}
pub mod player {
    pub mod main;
    pub mod cli_creator;
}

use std::error::Error;

use player::cli_creator::CliPlayerCreator;
use player::main::Player;
use gen_random::GenRandom;
use save::save_manager::{SaveManager, SavePathBuf};
use warrior::warrior_save_manager::WarriorSaveManager;
use warrior::Warrior;
use warrior::weapon::{Weapon, GiveWeapon};
use warrior::protection::{Protection, ProtectionKind, WearProtection};
use warrior::body::body_part::BodyPartKind;
use warrior::body::body_side::BodySide;
use tournament::Tournament;

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
    let builder = CliPlayerCreator::new();
    let player = Player::build(&builder)?;

    dbg!(&player);
    // let saver = WarriorSaveManager::build(SavePathBuf::from("saves"))?;
    // saver.save(contestants, SavePathBuf::from("contestants.save"))?;
//     let mut contestants: Vec<Warrior> = saver.build_from_save(&SavePathBuf::from("contestants.save"))?;

//     let mut tournament = Tournament::new(contestants);

//     tournament.fight_round(0);
    Ok(())
}
