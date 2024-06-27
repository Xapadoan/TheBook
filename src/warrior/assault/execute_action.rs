use crate::equipment::{HasRupture, RuptureTestResult, MayHaveRuptureDamage};
use crate::fight_mechanics::duration_damage::MayHaveDurationDamage;
use crate::fight_mechanics::{ApplyDamageModifier, CanMissAssaults, CanMissParries, RollDamage, TakeDamage};
use crate::fight_mechanics::parries_miss::ParriesMiss;
use crate::fight_mechanics::assaults_miss::AssaultsMiss;
use crate::fight_mechanics::TemporaryHandicap;
use crate::warrior::assault::Assault;
use crate::warrior::assault::attack::critical_hit::{CriticalHit, CriticalHitKind};
// use crate::warrior::assault::parry::parry_attempt::ParryAttemptResult;
use crate::warrior::assault::show_action::ShowAction;
// use crate::warrior::body::body_part::MayTargetBodyPart;
use crate::warrior::body::HasMutableBody;
// use crate::warrior::body::injury::{MayBeInjured, MayCauseInjury, TakeInjury};
// use crate::warrior::protection::Protectable;
use crate::warrior::weapon::{MayHaveMutableWeapon, MayHaveWeapon, TakeWeapon};
use crate::warrior::{HasBody, IsDead, IsUnconscious, Name, TakeReducedDamage};

use super::attack::can_be_attacked::CanBeAttacked;
// use super::attack::critical_hit::CriticalHitResult;
use super::parry::critical_parry::CriticalParryResult;
use super::parry::parry_attempt::ParryThreshold;

pub trait ExecuteAction {
    fn execute<A, V>(&mut self, assailant: &mut A, victim: &mut V)
    where
        A: RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + Name + HasBody + TakeDamage + TakeReducedDamage + CanBeAttacked + ParryThreshold + IsUnconscious + HasMutableBody + Assault + IsDead + MayHaveDurationDamage,
        V: RollDamage + Assault + CriticalHit + Name + MayHaveWeapon + IsUnconscious + HasMutableBody + CanMissAssaults + CanMissParries + MayHaveMutableWeapon + TakeWeapon + HasBody + TakeReducedDamage + TakeDamage + ParryThreshold + IsUnconscious + HasMutableBody + IsDead + MayHaveDurationDamage;
}

impl ExecuteAction for CriticalParryResult {
    fn execute<A, V>(&mut self, assailant: &mut A, victim: &mut V)
    where
        A: RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + Name + HasBody + TakeDamage + TakeReducedDamage + CanBeAttacked + ParryThreshold + IsUnconscious + HasMutableBody + Assault + IsDead + MayHaveDurationDamage,
        V: RollDamage + Assault + CriticalHit + Name + MayHaveWeapon + CanMissAssaults + CanMissParries + MayHaveMutableWeapon + TakeWeapon + HasBody + TakeReducedDamage + TakeDamage + ParryThreshold + IsUnconscious + HasMutableBody + IsDead + MayHaveDurationDamage,
    {
        match self {
            CriticalParryResult::AssailantBreaksWeapon(rupture_test_result) => {
                if assailant.weapon().is_some() {
                    match rupture_test_result {
                        RuptureTestResult::Fail => {
                            assailant.weapon_mut().unwrap().damage_rupture(u8::MAX);
                            assailant.take_weapon();
                        },
                        RuptureTestResult::Success => assailant.weapon_mut().unwrap().damage_rupture(1)
                    }
                }
            },
            CriticalParryResult::AssailantCriticalHit => {
                let mut crit_consequence = victim.critical_hit(assailant);
                crit_consequence.show(victim, assailant);
                crit_consequence.execute(victim, assailant);
            },
            CriticalParryResult::AssailantSelfCriticalHit => {
                let mut crit_consequence = victim.critical_hit(assailant);
                crit_consequence.show(victim, assailant);
                crit_consequence.execute(victim, assailant);
            },
            CriticalParryResult::AssailantDropsWeapon => {
                if assailant.weapon().is_some() {
                    assailant.take_weapon();
                }
            },
            CriticalParryResult::AssailantFalls => {
                assailant.will_miss_parries(ParriesMiss::new(2, String::from("he fell on the ground")));
                victim.assault(assailant);
                victim.assault(assailant);
            },
            CriticalParryResult::AssailantHit => {
                assailant.take_reduced_damage(victim.roll_damage());
            },
            CriticalParryResult::AssailantRepelled => {
                assailant.will_miss_assault(AssaultsMiss::new(1, format!("he was repelled by {}", victim.name())));
            },
            CriticalParryResult::AssailantTrips => {
                assailant.will_miss_parries(ParriesMiss::new(1, String::from("he is off balance")));
                victim.assault(assailant);
            },
            CriticalParryResult::RegularParry => {}
        }
    }
}
