use crate::dice::RollDamage;
use crate::modifiers::ApplyDamageModifier;
use crate::warrior::assault::Assault;
use crate::warrior::assault::attack::critical_hit::CriticalHit;
use crate::warrior::body::HasMutableBody;
use crate::warrior::duration_damage::MayHaveDurationDamage;
use crate::warrior::weapon::{MayHaveMutableWeapon, MayHaveWeapon, TakeWeapon};
use crate::warrior::{HasBody, IsDead, IsUnconscious, Name, TakeDamage, TakeReducedDamage};
use crate::warrior::temporary_handicap::parries_miss::CanMissParries;
use crate::warrior::temporary_handicap::assaults_miss::CanMissAssaults;

use super::attack::can_be_attacked::CanBeAttacked;
use super::damage_summary::DamageSummary;
use super::parry::parry_attempt::ParryThreshold;

pub trait ExecuteAction {
    fn execute<A, V>(&mut self, assailant: &mut A, victim: &mut V) -> DamageSummary
    where
        A: ApplyDamageModifier + CriticalHit + RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + Name + HasBody + TakeDamage + TakeReducedDamage + CanBeAttacked + ParryThreshold + IsUnconscious + HasMutableBody + Assault + IsDead + MayHaveDurationDamage,
        V: ApplyDamageModifier + CriticalHit + RollDamage + Assault + CriticalHit + Name + MayHaveWeapon + IsUnconscious + HasMutableBody + CanMissAssaults + CanMissParries + MayHaveMutableWeapon + TakeWeapon + HasBody + TakeReducedDamage + TakeDamage + ParryThreshold + IsUnconscious + HasMutableBody + IsDead + MayHaveDurationDamage;
}
