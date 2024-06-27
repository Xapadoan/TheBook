// use std::u8::MAX;

use crate::equipment::{HasRupture, RuptureTestResult};
use crate::warrior::assault::show_action::ShowAction;
// use crate::warrior::assault::attack::critical_hit::CriticalHit;
// use crate::warrior::assault::Assault;
use crate::warrior::body::HasBody;
use crate::warrior::{Name, Warrior};
use crate::warrior::weapon::{MayHaveMutableWeapon, MayHaveWeapon, TakeWeapon};
use crate::dice::Dice;
// use crate::fight_mechanics::assaults_miss::AssaultsMiss;
// use super::super::attack::AttackAttemptResult;
// use crate::fight_mechanics::fight_action::ExecuteFightActionResult;
// use crate::fight_mechanics::parries_miss::ParriesMiss;
// use super::ParryAttemptResult;
// use super::{CanMissAssaults, CanMissParries, CriticalHitOn, TemporaryHandicap};

#[derive(Debug)]
pub enum CriticalParryResult {
    RegularParry,
    AssailantRepelled,
    AssailantTrips,
    AssailantFalls,
    AssailantDropsWeapon,
    AssailantBreaksWeapon(RuptureTestResult),
    AssailantHit,
    AssailantCriticalHit,
    AssailantSelfCriticalHit,
}

pub trait CriticalParry {
    fn critical_parry<A: MayHaveWeapon + Name>(&self, assailant: &A) -> CriticalParryResult;
}

impl<T: Name + HasBody> CriticalParry for T {
    fn critical_parry<A: MayHaveWeapon + Name>(&self, assailant: &A) -> CriticalParryResult {
        let result = match Dice::D20.roll() {
            1 | 2 => CriticalParryResult::RegularParry,
            3..=5 => CriticalParryResult::AssailantRepelled,
            6 | 7 => CriticalParryResult::AssailantTrips,
            8 | 9 => CriticalParryResult::AssailantFalls,
            10..=12 => CriticalParryResult::AssailantDropsWeapon,
            13..=15 => if assailant.weapon().is_some() {
                CriticalParryResult::AssailantBreaksWeapon(assailant.weapon().unwrap().rupture_test())
            } else {
                CriticalParryResult::RegularParry
            }
            16..=18 => CriticalParryResult::AssailantHit,
            19 => CriticalParryResult::AssailantCriticalHit,
            20 => CriticalParryResult::AssailantSelfCriticalHit,
            other => panic!("D20 roll resulted in {other}"),
        };
        result.show(assailant, self);
        // result.execute(assailant, self);
        result
    }
}

// impl ShowFightActionResult for CriticalParryResult {
//     fn show_fight_action_result<A: MayHaveWeapon>(&self, assailant: &A, victim: &Warrior) {
//         match self {
//             CriticalParryResult::AssailantBreaksWeapon(rupture_test_result) => {
//                 if !assailant.weapon() {
//                     println!("{} has no weapon", assailant.name());
//                 } else {
//                     match rupture_test_result {
//                         RuptureTestResult::Fail => println!("{} broke {}'s weapon", victim.name(), assailant.name()),
//                         RuptureTestResult::Success => println!("{} damaged {}'s weapon", victim.name(), assailant.name()),
//                     }
//                 }
//             },
//             CriticalParryResult::AssailantCriticalHit => println!("{} finds a great counter", victim.name()),
//             CriticalParryResult::AssailantSelfCriticalHit => println!("{}'s weapon is repelled against him", assailant.name()),
//             CriticalParryResult::AssailantDropsWeapon => {
//                 if assailant.has_weapon() {
//                     println!("{} dropped his weapon", assailant.name())
//                 } else {
//                     println!("{} has no weapon", assailant.name())
//                 }
//             },
//             CriticalParryResult::AssailantFalls => println!("{} falls to the ground", assailant.name()),
//             CriticalParryResult::AssailantHit => println!("{} counters {}'s attack", victim.name(), assailant.name()),
//             CriticalParryResult::AssailantRepelled => println!("{} repelled {}", victim.name(), assailant.name()),
//             CriticalParryResult::AssailantTrips => println!("{} trips", assailant.name()),
//             CriticalParryResult::RegularParry => println!("{} parried successfully", victim.name())
//         }
//     }
// }

// impl ExecuteFightActionResult for CriticalParryResult {
//     fn execute(&mut self, assailant: &mut Warrior, victim: &mut Warrior) {
//         match self {
//             CriticalParryResult::AssailantBreaksWeapon(rupture_test_result) => {
//                 if assailant.weapon().is_some() {
//                     match rupture_test_result {
//                         RuptureTestResult::Fail => {
//                             assailant.weapon_mut().unwrap().damage_rupture(MAX);
//                             assailant.take_weapon();
//                         },
//                         RuptureTestResult::Success => assailant.weapon_mut().unwrap().damage_rupture(1)
//                     }
//                 }
//             },
//             CriticalParryResult::AssailantCriticalHit => {
//                 let mut crit_consequence = victim.critical_hit(assailant);
//                 crit_consequence.show(victim, assailant);
//                 crit_consequence.execute(victim, assailant);
//             },
//             CriticalParryResult::AssailantSelfCriticalHit => {
//                 let mut crit_consequence = victim.critical_hit(assailant);
//                 crit_consequence.show(victim, assailant);
//                 crit_consequence.execute(victim, assailant);
//             },
//             CriticalParryResult::AssailantDropsWeapon => {
//                 if assailant.weapon().is_some() {
//                     assailant.take_weapon();
//                 }
//             },
//             CriticalParryResult::AssailantFalls => {
//                 assailant.will_miss_parries(ParriesMiss::new(2, String::from("he fell on the ground")));
//                 victim.assault(assailant);
//                 victim.attack(assailant);
//             },
//             CriticalParryResult::AssailantHit => {
//                 // AttackAttemptResult::Success.execute(victim, assailant);
//                 println!("[WARN] damage phase not here yet")
//             },
//             CriticalParryResult::AssailantRepelled => {
//                 assailant.will_miss_assault(AssaultsMiss::new(1, format!("he was repelled by {}", victim.name())));
//             },
//             CriticalParryResult::AssailantTrips => {
//                 assailant.will_miss_parries(ParriesMiss::new(1, String::from("he is off balance")));
//                 victim.attack(assailant);
//             },
//             CriticalParryResult::RegularParry => {
//                 ParryAttemptResult::Success.execute(assailant, victim)
//             }
//         }
//     }
// }

