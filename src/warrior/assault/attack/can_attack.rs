use crate::dice::RollDamage;
use crate::modifiers::ApplyDamageModifier;
use crate::warrior::assault::damage_summary::DamageSummary;
use crate::warrior::assault::execute_action::ExecuteAction;
use crate::warrior::assault::parry::parry_attempt::ParryThreshold;
use crate::warrior::assault::show_action::ShowAction;
use crate::warrior::assault::Assault;
use crate::warrior::body::{HasBody, HasMutableBody};
use crate::warrior::duration_damage::MayHaveDurationDamage;
use crate::warrior::temporary_handicap::parries_miss::CanMissParries;
use crate::warrior::weapon::{MayHaveMutableWeapon, MayHaveWeapon, TakeWeapon};
use crate::warrior::{IsDead, IsUnconscious, HasName, TakeDamage, TakeReducedDamage};
use crate::warrior::temporary_handicap::assaults_miss::CanMissAssaults;

use super::can_be_attacked::CantBeAttackedReason;
use super::critical_hit::CriticalHit;
use super::CanBeAttacked;

#[derive(Debug)]
pub enum CantAttackReason {
    NoWeapon,
    IsDead,
    IsUnconscious,
    MustMissAssault,
    VictimCantBeAttacked(CantBeAttackedReason)
}

impl ShowAction for CantAttackReason {
    fn show<A, V>(&self, assailant: &A, victim: &V)
    where
        A: CanMissAssaults + MayHaveWeapon + HasName,
        V: MayHaveWeapon + HasName + HasBody + CanMissParries
    {
        match self {
            CantAttackReason::IsDead => println!("{} can't attack because he is dead", assailant.name()),
            CantAttackReason::IsUnconscious => println!("{} can't attack because he is unconscious", assailant.name()),
            CantAttackReason::MustMissAssault => println!("{} can't attack because {}", assailant.name(), assailant.must_miss_assault_reason()),
            CantAttackReason::NoWeapon => println!("{} can't attack attack he has no weapon", assailant.name()),
            CantAttackReason::VictimCantBeAttacked(reason) => reason.show(assailant, victim),
        }
    }
}

impl ExecuteAction for CanAttackResult {
    fn execute<A, V>(&mut self, assailant: &mut A, _: &mut V) -> DamageSummary
    where
        A: ApplyDamageModifier + CriticalHit + RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + HasName + HasBody + TakeDamage + TakeReducedDamage + CanBeAttacked + ParryThreshold + IsUnconscious + HasMutableBody + Assault + IsDead + MayHaveDurationDamage,
        V: ApplyDamageModifier + CriticalHit + RollDamage + Assault + HasName + MayHaveWeapon + IsUnconscious + HasMutableBody + CanMissAssaults + CanMissParries + MayHaveMutableWeapon + TakeWeapon + HasBody + TakeReducedDamage + TakeDamage + ParryThreshold + IsUnconscious + HasMutableBody + IsDead + MayHaveDurationDamage
    {
        match &self.reason {
            Some(reason) => match reason {
                CantAttackReason::MustMissAssault => assailant.miss_assault(),
                _ => {},
            },
            None => {},
        }
        DamageSummary::new(0)
    }
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

    pub fn reason(&self) -> Option<&CantAttackReason> {
        self.reason.as_ref()
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
        } else if self.weapon().is_none() {
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