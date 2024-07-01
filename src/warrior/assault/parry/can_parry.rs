use crate::dice::RollDamage;
use crate::modifiers::ApplyDamageModifier;
use crate::warrior::assault::attack::can_be_attacked::CanBeAttacked;
use crate::warrior::assault::attack::critical_hit::CriticalHit;
use crate::warrior::assault::damage_summary::DamageSummary;
use crate::warrior::assault::execute_action::ExecuteAction;
use crate::warrior::assault::Assault;
use crate::warrior::duration_damage::MayHaveDurationDamage;
use crate::warrior::temporary_handicap::assaults_miss::CanMissAssaults;
use crate::warrior::{IsDead, IsUnconscious, Name, TakeDamage, TakeReducedDamage};
use crate::warrior::assault::show_action::ShowAction;
use crate::warrior::body::{HasBody, HasMutableBody};
use crate::warrior::temporary_handicap::parries_miss::CanMissParries;
use crate::warrior::weapon::{MayHaveMutableWeapon, MayHaveWeapon, TakeWeapon};

use super::parry_attempt::ParryThreshold;

#[derive(Debug)]
pub enum CantParryReason {
    NoWeapon,
    IsDead,
    IsUnconscious,
    MustMissParry,
}

#[derive(Debug)]
pub struct CanParryResult {
    can_parry: bool,
    reason: Option<CantParryReason>
}

impl CanParryResult {
    pub fn can_parry(&self) -> bool {
        self.can_parry
    }

    pub fn reason(&self) -> Option<&CantParryReason> {
        self.reason.as_ref()
    }
}

pub trait CanParry {
    fn can_parry(&self) -> CanParryResult;
}

impl<T: MayHaveWeapon + CanMissParries + IsDead + IsUnconscious> CanParry for T {
    fn can_parry(&self) -> CanParryResult {
        if self.is_dead() {
            return CanParryResult { can_parry: false, reason: Some(CantParryReason::IsDead) };
        }
        if self.is_unconscious() {
            return CanParryResult { can_parry: false, reason: Some(CantParryReason::IsUnconscious) };
        }
        match self.weapon() {
            Some(_) => if self.must_miss_parry() {
                CanParryResult { can_parry: false, reason: Some(CantParryReason::MustMissParry) }
            } else {
                CanParryResult { can_parry: true, reason: None }
            },
            None => CanParryResult { can_parry: false, reason: Some(CantParryReason::NoWeapon) },
        }
    }
}

impl ShowAction for CantParryReason {
    fn show<A, V>(&self, _: &A, victim: &V)
    where
        A: MayHaveWeapon + Name,
        V: Name + HasBody + CanMissParries
    {
        match self {
            CantParryReason::IsDead => println!("{} can't parry because he is dead", victim.name()),
            CantParryReason::IsUnconscious => println!("{} can't parry because he is unconscious", victim.name()),
            CantParryReason::MustMissParry => println!("{} can't parry because {}", victim.name(), victim.must_miss_parry_reason()),
            CantParryReason::NoWeapon => println!("{} can't attack attack he has no weapon", victim.name()),
        }
    }
}
impl ExecuteAction for CanParryResult {
    fn execute<A, V>(&mut self, _: &mut A, victim: &mut V) -> DamageSummary
    where
        A: ApplyDamageModifier + CriticalHit + RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + Name + HasBody + TakeDamage + TakeReducedDamage + CanBeAttacked + ParryThreshold + IsUnconscious + HasMutableBody + Assault + IsDead + MayHaveDurationDamage,
        V: ApplyDamageModifier + CriticalHit + RollDamage + Assault + Name + MayHaveWeapon + IsUnconscious + HasMutableBody + CanMissAssaults + CanMissParries + MayHaveMutableWeapon + TakeWeapon + HasBody + TakeReducedDamage + TakeDamage + ParryThreshold + IsUnconscious + HasMutableBody + IsDead + MayHaveDurationDamage
    {
        match &self.reason {
            Some(reason) => match reason {
                CantParryReason::MustMissParry => victim.miss_parry(),
                _ => {},
            },
            None => {},
        }
        DamageSummary::new(0)
    }
}
