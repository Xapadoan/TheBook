use crate::warrior::Warrior;
use super::assaults_miss::AssaultsMiss;
use super::attack::AttackAttemptResult;
use super::fight_action::{ApplyFightActionResult, ShowFightActionResult};
use super::parries_miss::ParriesMiss;
use super::parry::ParryAttemptResult;
use super::{CanMissAssaults, CanMissParries, CriticalHit, TemporaryHandicap};

pub enum CriticalParryResult {
    RegularParry,
    AssailantRepelled,
    AssailantTrips,
    AssailantFalls,
    AssailantDropsWeapon,
    AssailantBreaksWeapon,
    AssailantHit,
    AssailantCriticalHit,
    AssailantCriticalHitBySelfWeapon,
}

impl ShowFightActionResult for CriticalParryResult {
    fn show_fight_action_result(&self, assailant: &Warrior, victim: &Warrior) {
        match self {
            CriticalParryResult::AssailantBreaksWeapon => println!("{} broke {}'s weapon", victim.name(), assailant.name()),
            CriticalParryResult::AssailantCriticalHit => println!("{} finds a great counter", victim.name()),
            CriticalParryResult::AssailantCriticalHitBySelfWeapon => println!("{}'s weapon is repelled against him", assailant.name()),
            CriticalParryResult::AssailantDropsWeapon => println!("{} dropped his weapon", assailant.name()),
            CriticalParryResult::AssailantFalls => println!("{} falls to the ground", assailant.name()),
            CriticalParryResult::AssailantHit => println!("{} counters {}'s attack", victim.name(), assailant.name()),
            CriticalParryResult::AssailantRepelled => println!("{} repelled {}", victim.name(), assailant.name()),
            CriticalParryResult::AssailantTrips => println!("{} trips", assailant.name()),
            CriticalParryResult::RegularParry => println!("{} parried successfully", victim.name())
        }
    }
}

impl ApplyFightActionResult for CriticalParryResult {
    fn apply_fight_action_result(&self, assailant: &mut Warrior, victim: &mut Warrior) {
        match self {
            CriticalParryResult::AssailantBreaksWeapon => {
                println!("[WARN]: Breaking weapon is not implemented")
            },
            CriticalParryResult::AssailantCriticalHit => {
                let crit_consequence = victim.critical_hit();
                crit_consequence.show_fight_action_result(victim, assailant);
                crit_consequence.apply_fight_action_result(victim, assailant);
            },
            CriticalParryResult::AssailantCriticalHitBySelfWeapon => {
                let crit_consequence = victim.critical_hit();
                crit_consequence.show_fight_action_result(victim, assailant);
                crit_consequence.apply_fight_action_result(victim, assailant);
            },
            CriticalParryResult::AssailantDropsWeapon => {
                println!("[WARN]: Dropping Weapon is not implemented")
            },
            CriticalParryResult::AssailantFalls => {
                assailant.will_miss_parries(ParriesMiss::new(2, String::from("he fell on the ground")));
                victim.attack(assailant);
                victim.attack(assailant);
            },
            CriticalParryResult::AssailantHit => {
                AttackAttemptResult::Success.apply_fight_action_result(victim, assailant);
            },
            CriticalParryResult::AssailantRepelled => {
                assailant.will_miss_assault(AssaultsMiss::new(1, format!("he was repelled by {}", victim.name())));
            },
            CriticalParryResult::AssailantTrips => {
                assailant.will_miss_parries(ParriesMiss::new(1, String::from("he is off balance")));
                victim.attack(assailant);
            },
            CriticalParryResult::RegularParry => {
                ParryAttemptResult::Success.apply_fight_action_result(assailant, victim)
            }
        }
    }
}

