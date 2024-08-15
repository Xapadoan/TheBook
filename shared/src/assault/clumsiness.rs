use std::u8;

use serde::{Deserialize, Serialize};

use crate::dice::Dice;
use crate::equipment::rupture::{Rupture, RuptureTestResult};
use crate::equipment::weapon::{OptionalMutableWeapon, Weapon};
use crate::random::Random;
use crate::warrior::body::body_part::{BodyPartKind, BodySide, FingerName, OptionalBodyPart};
use crate::warrior::body::injury::Injury;
use crate::warrior::body::HasBody;

use super::assault_consequence::IndividualConsequences;
use super::attack_success::ResolveAttackSuccess;
use super::common_traits::{ResolveBreakWeapon, ResolveDropWeapon, ResolveGougeRandomEye, ResolveMissAssaults};
use super::critical_hit::{DealCriticalHit, ResolveCriticalHit};
use super::common_traits::{DealDamages, TakeReducedDamage};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Clumsiness {
    RegularFail,
    Fall,
    DropWeapon,
    BreakWeapon,
    HitSelf,
    CriticalHitSelf,
    LoseEye,
    LoseFinger,
}

impl Random for Clumsiness {
    fn random() -> Self {
        match Dice::D20.roll() {
            1..=3 => Clumsiness::RegularFail,
            4..=7 => Clumsiness::Fall,
            8..=11 => Clumsiness::DropWeapon,
            12..=15 => Clumsiness::BreakWeapon,
            16..=18 => Clumsiness::HitSelf,
            19 => Clumsiness::CriticalHitSelf,
            20 =>  match Dice::D6.roll() {
                1 | 2 => Clumsiness::LoseEye,
                3..=6 => Clumsiness::LoseFinger,
                other => panic!("D6 roll resulted in {other}")
            },
            other => panic!("D20 roll resulted in {other}"),
        }
    }
}

pub trait ResolveClumsiness:
    DealDamages +
    ResolveAttackSuccess +
    DealCriticalHit +
    ResolveCriticalHit +
    ResolveGougeRandomEye +
    ResolveMissAssaults +
    ResolveDropWeapon +
    ResolveBreakWeapon +
    HasBody
{
    fn resolve_clumsiness(&self, clumsiness: Clumsiness, regular_fail_consequence: IndividualConsequences) -> IndividualConsequences {
        match clumsiness {
            Clumsiness::RegularFail => regular_fail_consequence,
            Clumsiness::Fall => self.resolve_miss_assaults(2),
            Clumsiness::DropWeapon => self.resolve_drop_weapon(),
            Clumsiness::BreakWeapon => self.resolve_break_weapon(),
            Clumsiness::HitSelf => self.resolve_hit_self(),
            Clumsiness::CriticalHitSelf => self.resolve_critical_hit_self(),
            Clumsiness::LoseEye => self.resolve_gouge_random_eye(0),
            Clumsiness::LoseFinger => self.resolve_sever_random_finger(),
        }
    }
    fn resolve_hit_self(&self) -> IndividualConsequences {
        self.resolve_hit(self.deal_damages())
    }
    fn resolve_critical_hit_self(&self) -> IndividualConsequences {
        let damages = self.deal_damages();
        let critical_hit = self.deal_critical_hit();
        self.resolve_critical_hit(damages, &critical_hit)
    }
    fn resolve_sever_random_finger(&self) -> IndividualConsequences {
        let affected_side = BodySide::random();
        if let None = self.body().body_part(&BodyPartKind::Arm(affected_side.clone())) {
            return IndividualConsequences::no_consequences();
        }
        if let None = self.body().body_part(&BodyPartKind::Hand(affected_side.clone())) {
            return IndividualConsequences::no_consequences();
        }
        let finger = FingerName::random();
        if let None = self.body().body_part(&BodyPartKind::Finger(affected_side.clone(), finger.clone())) {
            return IndividualConsequences::no_consequences()
        }
        IndividualConsequences::injury(0, Injury::FingerSevered(affected_side, finger))
    }
}

pub trait Clumsy:
    OptionalMutableWeapon +
    DealDamages +
    TakeReducedDamage +
    // TakeCriticalHitConsequences +
    DealCriticalHit
{
    fn fall_down(&mut self);
    fn drop_weapon(&mut self) -> Option<Weapon> {
        self.weapon_mut().take()
    }
    fn damage_weapon(&mut self) {
        if let None = self.weapon() {
            return;
        }
        let weapon = self.weapon_mut().as_mut().unwrap();
        match weapon.rupture_test() {
            RuptureTestResult::Success => weapon.damage_rupture(1),
            RuptureTestResult::Fail => weapon.damage_rupture(u8::MAX),
        }
    }
    fn resolve_hit_self(&mut self) {
        self.take_reduced_damages(self.deal_damages())
    }
    fn resolve_critical_hit_self(&mut self) {
        // self.take_damage(damages)
    }
    fn lose_eye(&mut self);
    fn lose_finger(&mut self);
}
