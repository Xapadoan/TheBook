use std::u8;

use serde::{Deserialize, Serialize};

use crate::dice::Dice;
use crate::random::Random;
use crate::warrior::body::body_part::{BodyPartKind, BodySide, FingerName, OptionalBodyPart};
use crate::warrior::body::injury::Injury;
use crate::warrior::body::HasBody;

use super::assault_consequence::IndividualConsequences;
use super::attack_success::ResolveAttackSuccess;
use super::common_traits::{ResolveBreakWeapon, ResolveDropWeapon, ResolveGougeRandomEye, ResolveMissAssaults};
use super::critical_hit::{DealCriticalHit, ResolveCriticalHit, ResolveCriticalHitSelf};
use super::common_traits::DealDamages;

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
    HasBody +
    ResolveCriticalHitSelf
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
        IndividualConsequences::injures(0, Injury::FingerSevered(affected_side, finger))
    }
}
