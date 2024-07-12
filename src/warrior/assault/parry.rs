use can_parry::{CanParry, CanParryResult};
use critical_parry::{CriticalParry, CriticalParryResult};
use parry_attempt::{ParryAttempt, ParryAttemptResult, ParryThreshold};
use serde::{Deserialize, Serialize};

use crate::dice::RollDamage;
use crate::modifiers::ApplyDamageModifier;
use crate::warrior::body::{HasBody, HasMutableBody};
use crate::warrior::duration_damage::MayHaveDurationDamage;
use crate::warrior::weapon::{MayHaveMutableWeapon, MayHaveWeapon, TakeWeapon};
use crate::warrior::temporary_handicap::parries_miss::CanMissParries;
use crate::warrior::temporary_handicap::assaults_miss::CanMissAssaults;
use crate::warrior::{HasName, IsDead, IsUnconscious, TakeDamage, TakeReducedDamage, Warrior};

use super::clumsiness::{Clumsiness, ClumsinessResult};
use super::damage_summary::DamageSummary;
use super::{attack::{can_be_attacked::CanBeAttacked, critical_hit::CriticalHit}, execute_action::ExecuteAction, show_action::ShowAction, Assault};

pub mod can_parry;
pub mod parry_attempt;
pub mod critical_parry;

#[derive(Debug, Serialize, Deserialize)]
pub struct ParryResult {
    can_parry: CanParryResult,
    parry_attempt: Option<ParryAttemptResult>,
    critical_success: Option<CriticalParryResult>,
    critical_failure: Option<ClumsinessResult>,
}

impl ParryResult {
    fn can_parry(&self) -> &CanParryResult {
        &self.can_parry
    }

    pub fn parry_attempt(&self) -> Option<&ParryAttemptResult> {
        self.parry_attempt.as_ref()
    }
}

pub trait Parry {
    fn parry<A: CriticalHit + RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + HasName + HasBody + TakeDamage + TakeReducedDamage + CanBeAttacked + ParryThreshold + IsUnconscious + HasMutableBody + Assault + IsDead + MayHaveDurationDamage>(&mut self, assailant: &mut A) -> ParryResult;
}

impl<T: CanParry + ParryAttempt + CriticalParry + Clumsiness> Parry for T {
    fn parry<A: CriticalHit + RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + HasName + HasBody + TakeDamage + TakeReducedDamage + CanBeAttacked + ParryThreshold + IsUnconscious + HasMutableBody + Assault + IsDead + MayHaveDurationDamage>(&mut self, assailant: &mut A) -> ParryResult {
        let can_parry = self.can_parry();
        if !can_parry.can_parry() {
            return ParryResult {
                can_parry,
                parry_attempt: None,
                critical_success: None,
                critical_failure: None,
            };
        }
        let parry_attempt = self.parry_attempt();
        match parry_attempt {
            ParryAttemptResult::CriticalFailure => {
                ParryResult {
                    can_parry,
                    parry_attempt: Some(parry_attempt),
                    critical_success: None,
                    critical_failure: Some(self.clumsiness())
                }
            },
            ParryAttemptResult::CriticalSuccess => ParryResult {
                can_parry,
                parry_attempt: Some(parry_attempt),
                critical_success: Some(self.critical_parry(assailant)),
                critical_failure: None,
            },
            _ => ParryResult {
                can_parry,
                parry_attempt: Some(parry_attempt),
                critical_success: None,
                critical_failure: None,
            }
        }
    }
}

impl ShowAction for ParryResult {
    fn show<A, V>(&self, assailant: &A, victim: &V)
    where
        A: MayHaveWeapon + HasName + CanMissAssaults,
        V: MayHaveWeapon + HasName + HasBody + CanMissParries
    {
        let parry_possibility = self.can_parry();
        if !parry_possibility.can_parry() {
            parry_possibility.reason().unwrap().show(assailant, victim);
            return;
        }
        let parry_attempt = self.parry_attempt();
        match parry_attempt.unwrap() {
            ParryAttemptResult::CriticalFailure => self.critical_failure.as_ref().unwrap().show(assailant, victim),
            ParryAttemptResult::Failure => println!("{} hits {}", assailant.name(), victim.name()),
            ParryAttemptResult::Success => println!("{} parried", victim.name()),
            ParryAttemptResult::CriticalSuccess => self.critical_success.as_ref().unwrap().show(assailant, victim),
        }
    }
}

impl ExecuteAction for ParryResult {
    fn execute(&mut self, assailant: &mut Warrior, victim: &mut Warrior) -> DamageSummary {
        let parry_possibility = self.can_parry();
        if !parry_possibility.can_parry() {
            return self.can_parry.execute(assailant, victim)
        }
        let parry_attempt = self.parry_attempt().unwrap();
        match parry_attempt {
            ParryAttemptResult::CriticalFailure => self.critical_failure.as_mut().unwrap().execute(assailant, victim),
            ParryAttemptResult::Failure => {
                let raw_damage = assailant.roll_damage();
                // println!("{} fails to parry and takes {} damage", victim.name(), raw_damage);
                let damage = victim.apply_damage_modifier(raw_damage);
                DamageSummary::new(damage)
            },
            ParryAttemptResult::Success => DamageSummary::new(0),
            ParryAttemptResult::CriticalSuccess => self.critical_success.as_mut().unwrap().execute(assailant, victim),
        }
    }
}