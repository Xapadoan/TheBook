pub mod can_attack;
pub mod can_be_attacked;
pub mod attack_attempt;
pub mod critical_hit;

use attack_attempt::{AttackAttempt, AttackAttemptResult};
use can_attack::{CanAttack, CanAttackResult};
use can_be_attacked::CanBeAttacked;
use critical_hit::{CriticalHit, CriticalHitResult};
use crate::{fight_mechanics::RollDamage, warrior::{body::HasBody, weapon::MayHaveWeapon, Name}};

#[derive(Debug)]
pub struct AttackResult {
    can_attack: CanAttackResult,
    attack_attempt: Option<AttackAttemptResult>,
    critical_success: Option<CriticalHitResult>
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
    fn attack<V: CanBeAttacked + HasBody + MayHaveWeapon + Name>(&mut self, victim: &mut V) -> AttackResult;
}

impl<A: CanAttack + AttackAttempt + CriticalHit + RollDamage> Attack for A {
    fn attack<V: CanBeAttacked + HasBody + MayHaveWeapon + Name>(&mut self, victim: &mut V) -> AttackResult {
        let can_attack = self.can_attack(victim);
        if !can_attack.can_attack() {
            return AttackResult {
                can_attack,
                attack_attempt: None,
                critical_success: None,
            }
        }
        let attack_attempt = self.attack_attempt();
        match attack_attempt {
            AttackAttemptResult::CriticalFailure => {
                println!("[WARN] Critical Hit Fail not implemented yet");
                AttackResult {
                    can_attack,
                    attack_attempt: Some(attack_attempt),
                    critical_success: None,
                }
            },
            AttackAttemptResult::CriticalSuccess => AttackResult {
                can_attack,
                attack_attempt: Some(attack_attempt),
                critical_success: Some(self.critical_hit(victim))
            },
            _ => AttackResult {
                can_attack,
                attack_attempt: Some(attack_attempt),
                critical_success: None,
            }
        }
    }
}
