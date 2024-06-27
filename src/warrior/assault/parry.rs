use can_parry::{CanParry, CanParryResult};
use critical_parry::{CriticalParry, CriticalParryResult};
use parry_attempt::{ParryAttempt, ParryAttemptResult};

use crate::{fight_mechanics::TakeDamage, warrior::{weapon::MayHaveWeapon, Name}};

pub mod can_parry;
pub mod parry_attempt;
pub mod critical_parry;

#[derive(Debug)]
pub struct ParryResult {
    can_parry: CanParryResult,
    parry_attempt: Option<ParryAttemptResult>,
    critical_success: Option<CriticalParryResult>,
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
    fn parry<A: MayHaveWeapon + TakeDamage + Name>(&mut self, assailant: &mut A) -> ParryResult;
}

impl<T: CanParry + ParryAttempt + CriticalParry> Parry for T {
    fn parry<A: MayHaveWeapon + TakeDamage + Name>(&mut self, assailant: &mut A) -> ParryResult {
        let can_parry = self.can_parry();
        if !can_parry.can_parry() {
            return ParryResult {
                can_parry,
                parry_attempt: None,
                critical_success: None,
            };
        }
        let parry_attempt = self.parry_attempt();
        match parry_attempt {
            ParryAttemptResult::CriticalFailure => {
                println!("[WARN] Critical Parry fail not implemented yet");
                ParryResult {
                    can_parry,
                    parry_attempt: Some(parry_attempt),
                    critical_success: None,
                }
            },
            ParryAttemptResult::CriticalSuccess => ParryResult {
                can_parry,
                parry_attempt: Some(parry_attempt),
                critical_success: Some(self.critical_parry(assailant))
            },
            _ => ParryResult {
                can_parry,
                parry_attempt: Some(parry_attempt),
                critical_success: None,
            }
        }
    }
}