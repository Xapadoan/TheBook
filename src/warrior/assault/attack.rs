pub mod can_attack;
pub mod can_be_attacked;
pub mod attack_attempt;
pub mod critical_hit;

use attack_attempt::{AttackAttempt, AttackAttemptResult};
use can_attack::{CanAttack, CanAttackResult};
use can_be_attacked::CanBeAttacked;
use critical_hit::{CriticalHit, CriticalHitResult};

use crate::dice::RollDamage;
use crate::modifiers::ApplyDamageModifier;
use crate::warrior::body::{HasBody, HasMutableBody};
use crate::warrior::duration_damage::MayHaveDurationDamage;
use crate::warrior::weapon::{MayHaveMutableWeapon, MayHaveWeapon, TakeWeapon};
use crate::warrior::{IsDead, IsUnconscious, Name, TakeDamage, TakeReducedDamage};
use crate::warrior::temporary_handicap::parries_miss::CanMissParries;
use crate::warrior::temporary_handicap::assaults_miss::CanMissAssaults;

use super::clumsiness::{Clumsiness, ClumsinessResult};
use super::damage_summary::DamageSummary;
use super::{execute_action::ExecuteAction, parry::parry_attempt::ParryThreshold, show_action::ShowAction, Assault};

#[derive(Debug)]
pub struct AttackResult {
    can_attack: CanAttackResult,
    attack_attempt: Option<AttackAttemptResult>,
    critical_success: Option<CriticalHitResult>,
    critical_fail: Option<ClumsinessResult>,
}

impl AttackResult {
    fn can_attack(&self) -> &CanAttackResult {
        &self.can_attack
    }

    pub fn attack_attempt(&self) -> Option<&AttackAttemptResult> {
        self.attack_attempt.as_ref()
    }
}

pub trait Attack {
    fn attack<V: Assault + CriticalHit + MayHaveDurationDamage + IsDead + ParryThreshold + TakeReducedDamage + TakeWeapon + MayHaveMutableWeapon + CanMissParries + CanMissAssaults + HasMutableBody + IsUnconscious + MayHaveWeapon + RollDamage + TakeDamage + Name + HasBody>(&mut self, victim: &mut V) -> AttackResult;
}

impl<A: CanAttack + AttackAttempt + CriticalHit + RollDamage + MayHaveWeapon + Name + CanMissAssaults + Clumsiness> Attack for A {
    fn attack<V: Assault + CriticalHit + MayHaveDurationDamage + IsDead + ParryThreshold + TakeReducedDamage + TakeWeapon + MayHaveMutableWeapon + CanMissParries + CanMissAssaults + HasMutableBody + IsUnconscious + MayHaveWeapon + RollDamage + TakeDamage + Name + HasBody>(&mut self, victim: &mut V) -> AttackResult {
        let can_attack = self.can_attack(victim);
        if !can_attack.can_attack() {
            return AttackResult {
                can_attack,
                attack_attempt: None,
                critical_success: None,
                critical_fail: None,
            }
        }
        let attack_attempt = self.attack_attempt();
        match attack_attempt {
            AttackAttemptResult::CriticalFailure => {
                AttackResult {
                    can_attack,
                    attack_attempt: Some(attack_attempt),
                    critical_success: None,
                    critical_fail: Some(self.clumsiness()),
                }
            },
            AttackAttemptResult::CriticalSuccess => AttackResult {
                can_attack,
                attack_attempt: Some(attack_attempt),
                critical_success: Some(self.critical_hit(victim)),
                critical_fail: None,
            },
            _ => AttackResult {
                can_attack,
                attack_attempt: Some(attack_attempt),
                critical_success: None,
                critical_fail: None,
            }
        }
    }
}

impl ShowAction for AttackResult {
    fn show<A, V>(&self, assailant: &A, victim: &V)
        where
            A: MayHaveWeapon + Name + CanMissAssaults,
            V: MayHaveWeapon + Name + HasBody + CanMissParries
    {
        let attack_possibility = self.can_attack();
        if !attack_possibility.can_attack() {
            attack_possibility.reason().unwrap().show(assailant, victim);
            return;
        }
        let attack_attempt = self.attack_attempt().unwrap();
        match attack_attempt {
            AttackAttemptResult::CriticalFailure => self.critical_fail.as_ref().unwrap().show(assailant, victim),
            AttackAttemptResult::Failure => println!("{} misses", assailant.name()),
            AttackAttemptResult::Success => println!("{} is landing a hit on {}", assailant.name(), victim.name()),
            AttackAttemptResult::CriticalSuccess => self.critical_success.as_ref().unwrap().show(assailant, victim),
        }
    }
}

impl ExecuteAction for AttackResult {
    fn execute<A, V>(&mut self, assailant: &mut A, victim: &mut V) -> DamageSummary
    where
        A: ApplyDamageModifier + CriticalHit + RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + Name + HasBody + TakeDamage + TakeReducedDamage + CanBeAttacked + ParryThreshold + IsUnconscious + HasMutableBody + Assault + IsDead + MayHaveDurationDamage,
        V: ApplyDamageModifier + CriticalHit + RollDamage + CanMissParries + Assault + CriticalHit + Name + MayHaveWeapon + IsUnconscious + HasMutableBody + CanMissAssaults + CanMissParries + MayHaveMutableWeapon + TakeWeapon + HasBody + TakeReducedDamage + TakeDamage + ParryThreshold + IsUnconscious + HasMutableBody + IsDead + MayHaveDurationDamage
    {
        let attack_possibility = self.can_attack();
        if !attack_possibility.can_attack() {
            return self.can_attack.execute(assailant, victim)
        }
        let attack_attempt = self.attack_attempt().unwrap();
        match attack_attempt {
            AttackAttemptResult::CriticalFailure => self.critical_fail.as_mut().unwrap().execute(assailant, victim),
            AttackAttemptResult::Failure => DamageSummary::new(0),
            AttackAttemptResult::Success => DamageSummary::new(0),
            AttackAttemptResult::CriticalSuccess => self.critical_success.as_mut().unwrap().execute(assailant, victim),
        }
    }
}
