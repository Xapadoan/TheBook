use crate::warrior::weapon::MayHaveWeapon;
use crate::warrior::{IsDead, IsUnconscious};
use crate::fight_mechanics::CanMissAssaults;

use super::can_be_attacked::CantBeAttackedReason;
use super::CanBeAttacked;

#[derive(Debug)]
pub enum CantAttackReason {
    NoWeapon,
    IsDead,
    IsUnconscious,
    MustMissAssault,
    VictimCantBeAttacked(CantBeAttackedReason)
}

#[derive(Debug)]
pub struct CanAttackResult {
    can_attack: bool,
    reason: Option<CantAttackReason>,
}

impl CanAttackResult {
    pub fn can_attack(&self) -> bool {
        self.can_attack
    }

    pub fn reason(self) -> Option<CantAttackReason> {
        self.reason
    }
}
pub trait CanAttack {
    fn can_attack<V: CanBeAttacked>(&self, victim: &V) -> CanAttackResult;
}

impl<T: MayHaveWeapon + IsDead + IsUnconscious + CanMissAssaults> CanAttack for T {
    fn can_attack<V: CanBeAttacked>(&self, victim: &V) -> CanAttackResult {
        let victim_can_be_attacked = victim.can_be_attacked();
        if !victim_can_be_attacked.can_be_attacked() {
            CanAttackResult {
                can_attack: false,
                reason: Some(CantAttackReason::VictimCantBeAttacked(victim_can_be_attacked.reason().unwrap()))
            }
        } else if !self.weapon().is_none() {
            CanAttackResult {
                can_attack: false,
                reason: Some(CantAttackReason::NoWeapon)
            }
        } else if self.is_dead() {
            CanAttackResult {
                can_attack: false,
                reason: Some(CantAttackReason::IsDead)
            }
        } else if self.is_unconscious() {
            CanAttackResult {
                can_attack: false,
                reason: Some(CantAttackReason::IsUnconscious)
            }
        } else if self.must_miss_assault() {
            CanAttackResult {
                can_attack: false,
                reason: Some(CantAttackReason::MustMissAssault)
            }
        } else {
            CanAttackResult {
                can_attack: true,
                reason: None
            }
        }
    }
}