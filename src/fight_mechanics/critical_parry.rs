use std::u8::MAX;

use crate::equipment::{HasRupture, RuptureTestResult};
use crate::warrior::Warrior;
use crate::weapon::CanHaveWeapon;
use crate::dice::Dice;
use super::assaults_miss::AssaultsMiss;
use super::attack::AttackAttemptResult;
use super::fight_action::{ExecuteFightActionResult, ShowFightActionResult};
use super::parries_miss::ParriesMiss;
use super::parry::ParryAttemptResult;
use super::{CanMissAssaults, CanMissParries, CriticalHitOn, TemporaryHandicap};

pub trait CriticalParry {
    fn critical_parry(&self, assailant: &Warrior) -> CriticalParryResult {
        match Dice::D20.roll() {
            1 | 2 => CriticalParryResult::RegularParry,
            3..=5 => CriticalParryResult::AssailantRepelled,
            6 | 7 => CriticalParryResult::AssailantTrips,
            8 | 9 => CriticalParryResult::AssailantFalls,
            10..=12 => CriticalParryResult::AssailantDropsWeapon,
            13..=15 => if assailant.has_weapon() {
                CriticalParryResult::AssailantBreaksWeapon(assailant.weapon().unwrap().rupture_test())
            } else {
                CriticalParryResult::RegularParry
            }
            16..=18 => CriticalParryResult::AssailantHit,
            19 => CriticalParryResult::AssailantCriticalHit,
            20 => CriticalParryResult::AssailantSelfCriticalHit,
            other => panic!("D20 roll resulted in {other}"),
        }
    }
}

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

impl ShowFightActionResult for CriticalParryResult {
    fn show_fight_action_result(&self, assailant: &Warrior, victim: &Warrior) {
        match self {
            CriticalParryResult::AssailantBreaksWeapon(rupture_test_result) => {
                if !assailant.has_weapon() {
                    println!("{} has no weapon", assailant.name());
                } else {
                    match rupture_test_result {
                        RuptureTestResult::Fail => println!("{} broke {}'s weapon", victim.name(), assailant.name()),
                        RuptureTestResult::Success => println!("{} damaged {}'s weapon", victim.name(), assailant.name()),
                    }
                }
            },
            CriticalParryResult::AssailantCriticalHit => println!("{} finds a great counter", victim.name()),
            CriticalParryResult::AssailantSelfCriticalHit => println!("{}'s weapon is repelled against him", assailant.name()),
            CriticalParryResult::AssailantDropsWeapon => {
                if assailant.has_weapon() {
                    println!("{} dropped his weapon", assailant.name())
                } else {
                    println!("{} has no weapon", assailant.name())
                }
            },
            CriticalParryResult::AssailantFalls => println!("{} falls to the ground", assailant.name()),
            CriticalParryResult::AssailantHit => println!("{} counters {}'s attack", victim.name(), assailant.name()),
            CriticalParryResult::AssailantRepelled => println!("{} repelled {}", victim.name(), assailant.name()),
            CriticalParryResult::AssailantTrips => println!("{} trips", assailant.name()),
            CriticalParryResult::RegularParry => println!("{} parried successfully", victim.name())
        }
    }
}

impl ExecuteFightActionResult for CriticalParryResult {
    fn execute(&mut self, assailant: &mut Warrior, victim: &mut Warrior) {
        match self {
            CriticalParryResult::AssailantBreaksWeapon(rupture_test_result) => {
                if assailant.has_weapon() {
                    match rupture_test_result {
                        RuptureTestResult::Fail => assailant.weapon_mut().unwrap().damage_rupture(MAX),
                        RuptureTestResult::Success => assailant.weapon_mut().unwrap().damage_rupture(1)
                    }
                }
            },
            CriticalParryResult::AssailantCriticalHit => {
                let mut crit_consequence = victim.critical_hit_on(assailant);
                crit_consequence.show_fight_action_result(victim, assailant);
                crit_consequence.execute(victim, assailant);
            },
            CriticalParryResult::AssailantSelfCriticalHit => {
                let mut crit_consequence = victim.critical_hit_on(assailant);
                crit_consequence.show_fight_action_result(victim, assailant);
                crit_consequence.execute(victim, assailant);
            },
            CriticalParryResult::AssailantDropsWeapon => {
                if assailant.has_weapon() {
                    assailant.drop_weapon();
                }
            },
            CriticalParryResult::AssailantFalls => {
                assailant.will_miss_parries(ParriesMiss::new(2, String::from("he fell on the ground")));
                victim.attack(assailant);
                victim.attack(assailant);
            },
            CriticalParryResult::AssailantHit => {
                AttackAttemptResult::Success.execute(victim, assailant);
            },
            CriticalParryResult::AssailantRepelled => {
                assailant.will_miss_assault(AssaultsMiss::new(1, format!("he was repelled by {}", victim.name())));
            },
            CriticalParryResult::AssailantTrips => {
                assailant.will_miss_parries(ParriesMiss::new(1, String::from("he is off balance")));
                victim.attack(assailant);
            },
            CriticalParryResult::RegularParry => {
                ParryAttemptResult::Success.execute(assailant, victim)
            }
        }
    }
}

