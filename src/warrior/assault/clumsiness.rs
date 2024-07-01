use crate::dice::{Dice, RollDamage};
use crate::equipment::{HasRupture, RuptureTestResult};
use crate::modifiers::ApplyDamageModifier;
use crate::warrior::body::body_part::{BodyPartKind, FingerName};
use crate::warrior::body::body_side::BodySide;
use crate::warrior::body::injury::{Injury, InjuryKind, MayBeInjured};
use crate::warrior::body::HasBody;
use crate::warrior::temporary_handicap::assaults_miss::{AssaultsMiss, CanMissAssaults};
use crate::warrior::temporary_handicap::parries_miss::CanMissParries;
use crate::warrior::weapon::{MayHaveMutableWeapon, MayHaveWeapon, TakeWeapon};
use crate::warrior::Name;

use super::attack::critical_hit::CriticalHit;
use super::damage_summary::DamageSummary;
use super::execute_action::ExecuteAction;
use super::show_action::ShowAction;

pub trait Clumsiness {
    fn clumsiness(&self) -> ClumsinessResult;
}

impl<T: MayHaveWeapon + RollDamage + ApplyDamageModifier + CriticalHit> Clumsiness for T {
    fn clumsiness(&self) -> ClumsinessResult {
        match Dice::D20.roll() {
            1..=3 => ClumsinessResult::RegularFail,
            4..=7 => ClumsinessResult::Fall,
            8..=11 => ClumsinessResult::DropWeapon,
            12..=15 => match self.weapon() {
                Some(weapon) => ClumsinessResult::BreakWeapon(weapon.rupture_test()),
                None => ClumsinessResult::RegularFail,
            },
            16..=18 => ClumsinessResult::HitSelf,
            19 => ClumsinessResult::CriticalHitSelf,
            20 => {
                let side = BodySide::random();
                match Dice::D6.roll() {
                    1 | 2 => ClumsinessResult::LoseEye(BodyPartKind::Eye(side)),
                    3..=6 => match Dice::D6.roll() {
                        1 | 2 => ClumsinessResult::LoseFinger(BodyPartKind::Finger(side, FingerName::Thumb)),
                        3 => ClumsinessResult::LoseFinger(BodyPartKind::Finger(side, FingerName::PointerFinger)),
                        4 => ClumsinessResult::LoseFinger(BodyPartKind::Finger(side, FingerName::MiddleFinger)),
                        5 => ClumsinessResult::LoseFinger(BodyPartKind::Finger(side, FingerName::RingFinger)),
                        6 => ClumsinessResult::LoseFinger(BodyPartKind::Finger(side, FingerName::PinkyFinger)),
                        other => panic!("D6 roll resulted in {other}")
                    },
                    other => panic!("D6 roll resulted in {other}")
                }
            },
            other => panic!("D20 roll resulted in {other}")
        }
    }
}

#[derive(Debug)]
pub enum ClumsinessResult {
    RegularFail,
    Fall,
    DropWeapon,
    BreakWeapon(RuptureTestResult),
    HitSelf,
    CriticalHitSelf,
    LoseEye(BodyPartKind),
    LoseFinger(BodyPartKind),
}

impl ShowAction for ClumsinessResult {
    fn show<A, V>(&self, assailant: &A, victim: &V)
    where
        A: Name + MayHaveWeapon + CanMissAssaults,
        V: Name + MayHaveWeapon + HasBody + CanMissParries
    {
        match self {
            ClumsinessResult::BreakWeapon(rupture_test_result) => match rupture_test_result {
                RuptureTestResult::Fail => println!("{}'s {} breaks when hitting {}'s {}", assailant.name(), assailant.weapon().unwrap(), victim.name(), victim.weapon().unwrap()),
                RuptureTestResult::Success => println!("{}'s {} hits the ground heavily", assailant.name(), assailant.weapon().unwrap()),
            },
            ClumsinessResult::CriticalHitSelf => println!("{} slips and fall on his {}", assailant.name(), assailant.weapon().unwrap()),
            ClumsinessResult::DropWeapon => println!("{}'s {} slips right off his hands", assailant.name(), assailant.weapon().unwrap()),
            ClumsinessResult::Fall => println!("{} trips and fall on the ground", assailant.name()),
            ClumsinessResult::HitSelf => println!("{} slips an hurt himself", assailant.name()),
            ClumsinessResult::LoseEye(eye) => println!("{} hits is own {eye}", assailant.name()),
            ClumsinessResult::LoseFinger(finger) => println!("{} somehow manages to cut his {finger}", assailant.name()),
            ClumsinessResult::RegularFail => println!("{} trips but manages to balance himself", assailant.name()),
        }
    }
}

impl ExecuteAction for ClumsinessResult {
    fn execute<A, V>(&mut self, assailant: &mut A, _: &mut V) -> super::damage_summary::DamageSummary
    where
        A: ApplyDamageModifier + CriticalHit + RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + Name + HasBody + crate::warrior::TakeDamage + crate::warrior::TakeReducedDamage + super::parry::parry_attempt::ParryThreshold + crate::warrior::IsUnconscious + crate::warrior::body::HasMutableBody + crate::warrior::IsDead + crate::warrior::duration_damage::MayHaveDurationDamage + super::Assault + super::attack::can_be_attacked::CanBeAttacked,
        V: ApplyDamageModifier + CriticalHit + RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + Name + HasBody + crate::warrior::TakeDamage + crate::warrior::TakeReducedDamage + super::parry::parry_attempt::ParryThreshold + crate::warrior::IsUnconscious + crate::warrior::body::HasMutableBody + crate::warrior::IsDead + crate::warrior::duration_damage::MayHaveDurationDamage + super::Assault + crate::warrior::IsUnconscious + crate::warrior::body::HasMutableBody
    {
        let mut damage_summary = DamageSummary::new(0);
        match self {
            ClumsinessResult::BreakWeapon(rupture_test_result) => {
                let weapon = assailant.weapon_mut().unwrap();
                match rupture_test_result {
                    RuptureTestResult::Fail => weapon.damage_rupture(u8::MAX),
                    RuptureTestResult::Success => weapon.damage_rupture(1),
                }
            },
            ClumsinessResult::CriticalHitSelf => {
                let mut critical_hit_result = assailant.critical_hit(assailant);
                critical_hit_result.show(assailant, assailant);
                let inter_damage_summary = critical_hit_result.self_inflict(assailant);
                damage_summary.merge(inter_damage_summary, true);
            },
            ClumsinessResult::DropWeapon => { assailant.take_weapon(); }
            ClumsinessResult::Fall => {
                assailant.will_miss_assault(
                    AssaultsMiss::new(2, format!("fell on the ground"))
                )
            },
            ClumsinessResult::HitSelf => {
                let damage = assailant.apply_damage_modifier(assailant.roll_damage());
                damage_summary.add_damage_to_assailant(damage);
            },
            ClumsinessResult::LoseEye(eye) => {
                assailant.body_mut().body_part_mut(eye).add_injury(
                    Injury::new(
                        InjuryKind::Gouged,
                        -1,
                        -2,
                        // String::from("he gouged is own eye")
                    )
                )
            },
            ClumsinessResult::LoseFinger(finger) => {
                assailant.body_mut().body_part_mut(finger).add_injury(
                    Injury::new(
                        InjuryKind::Severed,
                        0,
                        0,
                        // String::from("he cut is own finger")
                    )
                )
            },
            ClumsinessResult::RegularFail => {},
        }
        damage_summary
    }
}
