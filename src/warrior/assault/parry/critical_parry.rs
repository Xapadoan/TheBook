use crate::dice::RollDamage;
use crate::equipment::{HasRupture, RuptureTestResult};
use crate::modifiers::ApplyDamageModifier;
use crate::warrior::assault::attack::can_be_attacked::CanBeAttacked;
use crate::warrior::assault::attack::critical_hit::CriticalHit;
use crate::warrior::assault::damage_summary::DamageSummary;
use crate::warrior::assault::execute_action::ExecuteAction;
use crate::warrior::assault::show_action::ShowAction;
use crate::warrior::assault::Assault;
use crate::warrior::body::{HasBody, HasMutableBody};
use crate::warrior::duration_damage::MayHaveDurationDamage;
use crate::warrior::{IsDead, IsUnconscious, Name, TakeDamage, TakeReducedDamage};
use crate::warrior::weapon::{MayHaveMutableWeapon, MayHaveWeapon, TakeWeapon};
use crate::warrior::temporary_handicap::parries_miss::{CanMissParries, ParriesMiss};
use crate::warrior::temporary_handicap::assaults_miss::{CanMissAssaults, AssaultsMiss};
use crate::dice::Dice;

use super::parry_attempt::ParryThreshold;

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
    fn critical_parry<A: CriticalHit + RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + Name + HasBody + TakeDamage + TakeReducedDamage + CanBeAttacked + ParryThreshold + IsUnconscious + HasMutableBody + Assault + IsDead + MayHaveDurationDamage>(&mut self, assailant: &mut A) -> CriticalParryResult;
}

impl<T: CriticalHit + RollDamage + Assault + CriticalHit + Name + MayHaveWeapon + CanMissAssaults + CanMissParries + MayHaveMutableWeapon + TakeWeapon + HasBody + TakeReducedDamage + TakeDamage + ParryThreshold + IsUnconscious + HasMutableBody + IsDead + MayHaveDurationDamage> CriticalParry for T {
    fn critical_parry<A: CriticalHit + RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + Name + HasBody + TakeDamage + TakeReducedDamage + CanBeAttacked + ParryThreshold + IsUnconscious + HasMutableBody + Assault + IsDead + MayHaveDurationDamage>(&mut self, assailant: &mut A) -> CriticalParryResult {
        match Dice::D20.roll() {
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
        }
    }
}

impl ExecuteAction for CriticalParryResult {
    fn execute<A, V>(&mut self, assailant: &mut A, victim: &mut V) -> DamageSummary
    where
        A: ApplyDamageModifier + CriticalHit + RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + Name + HasBody + TakeDamage + TakeReducedDamage + CanBeAttacked + ParryThreshold + IsUnconscious + HasMutableBody + Assault + IsDead + MayHaveDurationDamage,
        V: ApplyDamageModifier + CriticalHit + RollDamage + Assault + CriticalHit + Name + MayHaveWeapon + CanMissAssaults + CanMissParries + MayHaveMutableWeapon + TakeWeapon + HasBody + TakeReducedDamage + TakeDamage + ParryThreshold + IsUnconscious + HasMutableBody + IsDead + MayHaveDurationDamage,
    {
        let mut damage_summary = DamageSummary::new(0);
        match self {
            CriticalParryResult::AssailantBreaksWeapon(rupture_test_result) => {
                if assailant.weapon().is_some() {
                    match rupture_test_result {
                        RuptureTestResult::Fail => {
                            assailant.weapon_mut().unwrap().damage_rupture(u8::MAX);
                            assailant.take_weapon();
                        },
                        RuptureTestResult::Success => assailant.weapon_mut().unwrap().damage_rupture(0)
                    }
                }
            },
            CriticalParryResult::AssailantCriticalHit => {
                let mut crit_consequence = victim.critical_hit(assailant);
                crit_consequence.show(victim, assailant);
                let inter_damage_summary = crit_consequence.execute(victim, assailant);
                damage_summary.merge(inter_damage_summary, true);
            },
            CriticalParryResult::AssailantSelfCriticalHit => {
                let mut crit_consequence = victim.critical_hit(assailant);
                crit_consequence.show(victim, assailant);
                let inter_damage_summary = crit_consequence.execute(victim, assailant);
                damage_summary.merge(inter_damage_summary, true);
            },
            CriticalParryResult::AssailantDropsWeapon => {
                if assailant.weapon().is_some() {
                    assailant.take_weapon();
                }
            },
            CriticalParryResult::AssailantFalls => {
                assailant.will_miss_parries(ParriesMiss::new(1, String::from("he fell on the ground")));
                victim.assault(assailant);
                victim.assault(assailant);
            },
            CriticalParryResult::AssailantHit => {
                assailant.take_reduced_damage(victim.roll_damage());
            },
            CriticalParryResult::AssailantRepelled => {
                assailant.will_miss_assault(AssaultsMiss::new(0, format!("he was repelled by {}", victim.name())));
            },
            CriticalParryResult::AssailantTrips => {
                assailant.will_miss_parries(ParriesMiss::new(0, String::from("he is off balance")));
                victim.assault(assailant);
            },
            CriticalParryResult::RegularParry => {}
        };
        damage_summary
    }
}

impl ShowAction for CriticalParryResult {
    fn show<A, V>(&self, assailant: &A, victim: &V)
    where
        A: MayHaveWeapon + Name,
        V: HasBody + Name
    {
        println!("{} parries perfectly", victim.name());
        match self {
            CriticalParryResult::AssailantBreaksWeapon(rupture_test_result) => {
                if assailant.weapon().is_none() {
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
                if assailant.weapon().is_some() {
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
