use std::u8;

use rand::Rng;

use crate::assault::assault_consequence::{ArmorDamages, IndividualConsequences};
use crate::assault::duration_damages::DurationDamages;
use crate::assault::common_traits::{DealDamages, ResolveGougeRandomEye};
use crate::equipment::protection::OptionalMutableProtection;
use crate::equipment::rupture::{Rupture, RuptureTestResult};
use crate::random::Random;
use crate::warrior::body::body_part::{BodyPartKind, BodySide, OptionalBodyPart, PROTECTABLE_BODY_PARTS};
use crate::warrior::body::injury::Injury;
use crate::warrior::body::HasMutableBody;

use super::{CriticalHit, DealCriticalHit};

//sever only
pub trait ResolveCriticalHit:
    HasMutableBody +
    ResolveGougeRandomEye +
{
    fn resolve_critical_hit(&self, damages: u8, critical_hit: &CriticalHit) -> IndividualConsequences {
        match critical_hit {
            CriticalHit::DeepIncision => self.resolve_raw_damages(damages + 1),
            CriticalHit::ReallyDeepIncision => self.resolve_raw_damages(damages + 2),
            CriticalHit::ImpressiveWoundAndArmorDamage => self.resolve_damage_random_armor_piece(
                damages + 3,
                1,
            ),
            CriticalHit::PreciseHitAndArmorDamage => self.resolve_damage_random_armor_piece(
                damages + 4,
                2,
            ),
            CriticalHit::AccurateHeavyBlowAndArmorDamage => self.resolve_damage_random_armor_piece(
                damages + 5,
                3,
            ),
            CriticalHit::PartOfTheArmorIsDestroyed => self.resolve_damage_random_armor_piece(
                damages,
                u8::MAX,
            ),
            CriticalHit::GougedEye => self.resolve_gouge_random_eye(damages),
            CriticalHit::SeveredHand => self.resolve_sever_random_hand(damages),
            CriticalHit::SeveredFoot => self.resolve_sever_random_foot(damages),
            CriticalHit::SeveredArm => self.resolve_sever_random_arm(damages),
            CriticalHit::SeveredLeg => self.resolve_sever_random_leg(damages),
            CriticalHit::WoundedGenitals => self.resolve_wound_genitals(damages),
            CriticalHit::VitalOrganDamage => self.resolve_duration_damage(damages + 9),
            CriticalHit::HeartInjury => self.resolve_death(),
            CriticalHit::SeriousHeadWound => self.resolve_death(),
            CriticalHit::ImpressiveBruise => self.resolve_raw_damages(damages + 1),
            CriticalHit::ImpressiveBruiseAndLimbDislocation => self.resolve_raw_damages(damages + 2),
            CriticalHit::RibFacture => self.resolve_raw_damages(damages + 2),
            CriticalHit::KneeDislocation => self.resolve_dislocate_random_knee(damages),
            CriticalHit::BrokenHand => self.resolve_break_random_hand(damages),
            CriticalHit::SmashedFoot => self.resolve_smash_random_foot(damages),
            CriticalHit::BrokenArm => self.resolve_break_random_arm(damages),
            CriticalHit::BrokenLeg => self.resolve_break_random_leg(damages),
            CriticalHit::CrushedGenitals => self.resolve_crush_genitals(damages),
            CriticalHit::KnockedOut => self.resolve_knock_out(damages),
            CriticalHit::OpenSkullFacture => self.resolve_death(),
            CriticalHit::VitalOrganCrushed => self.resolve_death(),
        }
    }
    fn resolve_damage_random_armor_piece(&self, damages: u8, rupture_damages: u8) -> IndividualConsequences {
        let mut armored_body_parts: Vec<BodyPartKind> = Vec::new();
        for body_part_kind in PROTECTABLE_BODY_PARTS {
            if let Some(body_part) = self.body().body_part(&body_part_kind) {
                if let Some(_) = body_part.protection() {
                    armored_body_parts.push(body_part_kind);
                }
            }
        }
        if armored_body_parts.len() < 1 {
            IndividualConsequences::only_damages(damages)
        } else if armored_body_parts.len() == 1 {
            IndividualConsequences::damage_armor(
                damages,
                ArmorDamages::new(rupture_damages, armored_body_parts[0].clone()),
            )
        } else {
            let random_index = rand::thread_rng().gen_range(0..armored_body_parts.len() - 1);
            IndividualConsequences::damage_armor(
                damages,
                ArmorDamages::new(rupture_damages,armored_body_parts[random_index].clone()),
            )
        }
    }
    fn resolve_sever_random_hand(&self, damages: u8) -> IndividualConsequences {
        let affected_side = BodySide::random();
        if self.body().body_part(&BodyPartKind::Arm(affected_side.clone())).is_none() {
            return IndividualConsequences::no_consequences()
        }
        match self.body().body_part(&BodyPartKind::Hand(affected_side.clone())) {
            None => IndividualConsequences::no_consequences(),
            Some(body_part) => {
                let injury = match affected_side {
                    BodySide::Right => Injury::RightHandSevered,
                    BodySide::Left => Injury::LeftHandSevered,
                };
                let total_damages = damages + 6;
                match body_part.protection() {
                    None => IndividualConsequences::injures(total_damages, injury),
                    Some(protection) => match protection.rupture_test() {
                        RuptureTestResult::Success => IndividualConsequences::damage_armor(
                            total_damages,
                            ArmorDamages::new(1, body_part.kind().clone()),
                        ),
                        RuptureTestResult::Fail => IndividualConsequences::injures(total_damages, injury)
                    }
                }
            }
        }
    }
    fn resolve_sever_random_foot(&self, damages: u8) -> IndividualConsequences {
        let affected_side = BodySide::random();
        if self.body().body_part(&BodyPartKind::Leg(affected_side.clone())).is_none() {
            return IndividualConsequences::no_consequences();
        }
        match self.body().body_part(&BodyPartKind::Foot(affected_side.clone())) {
            None => IndividualConsequences::no_consequences(),
            Some(body_part) => {
                let injury = Injury::FootSevered(affected_side);
                let total_damages = damages + 6;
                match body_part.protection() {
                    None => IndividualConsequences::injures(total_damages, injury),
                    Some(protection) => match protection.rupture_test() {
                        RuptureTestResult::Success => IndividualConsequences::damage_armor(
                            total_damages,
                            ArmorDamages::new(1, body_part.kind().clone())
                        ),
                        RuptureTestResult::Fail => IndividualConsequences::injures(total_damages, injury)
                    }
                }
            }
        }
    }
    fn resolve_sever_random_arm(&self, damages: u8) -> IndividualConsequences {
        let affected_side = BodySide::random();
        match self.body().body_part(&BodyPartKind::Arm(affected_side.clone())) {
            None => IndividualConsequences::no_consequences(),
            Some(body_part) => {
                let injury = match affected_side {
                    BodySide::Right => Injury::RightArmSevered,
                    BodySide::Left => Injury::LeftArmSevered,
                };
                let total_damages = damages + 7;
                match body_part.protection() {
                    None => IndividualConsequences::injures(total_damages, injury),
                    Some(protection) => match protection.rupture_test() {
                        RuptureTestResult::Success => IndividualConsequences::damage_armor(
                            total_damages,
                            ArmorDamages::new(1, body_part.kind().clone()),
                        ),
                        RuptureTestResult::Fail => IndividualConsequences::injures(total_damages, injury)
                    }
                }
            }
        }
    }
    fn resolve_sever_random_leg(&self, damages: u8) -> IndividualConsequences {
        let affected_side = BodySide::random();
        match self.body().body_part(&BodyPartKind::Leg(affected_side.clone())) {
            None => IndividualConsequences::no_consequences(),
            Some(body_part) => {
                let injury = Injury::OneLegSevered(affected_side);
                let total_damages = damages + 8;
                match body_part.protection() {
                    None => IndividualConsequences::injures(total_damages, injury),
                    Some(protection) => match protection.rupture_test() {
                        RuptureTestResult::Success => IndividualConsequences::damage_armor(
                            total_damages,
                            ArmorDamages::new(1, body_part.kind().clone()),
                        ),
                        RuptureTestResult::Fail => IndividualConsequences::injures(total_damages, injury)
                    }
                }
            }
        }
    }
    fn resolve_dislocate_random_knee(&self, damages: u8) -> IndividualConsequences {
        let affected_side = BodySide::random();
        match self.body().body_part(&BodyPartKind::Leg(affected_side.clone())) {
            None => IndividualConsequences::no_consequences(),
            Some(_) => {
                match self.body().body_part(&BodyPartKind::Knee(affected_side.clone())) {
                    None => IndividualConsequences::no_consequences(),
                    Some(knee) => if knee.is_broken() {
                        IndividualConsequences::only_damages(damages + 3)
                    } else {
                        IndividualConsequences::injures(damages + 3, Injury::KneeDislocated(affected_side))
                    }
                }
            }
        }
    }
    fn resolve_break_random_hand(&self, damages: u8) -> IndividualConsequences {
        let affected_side = BodySide::random();
        match self.body().body_part(&BodyPartKind::Arm(affected_side.clone())) {
            None => IndividualConsequences::no_consequences(),
            Some(_) => match self.body().body_part(&BodyPartKind::Hand(affected_side.clone())) {
                None => IndividualConsequences::no_consequences(),
                Some(hand) => {
                    let injury = match affected_side {
                        BodySide::Right => Injury::RightHandBroken,
                        BodySide::Left => Injury::LeftHandBroken,
                    };
                    if hand.is_broken() {
                        IndividualConsequences::only_damages(damages + 3)
                    } else {
                        IndividualConsequences::injures(damages + 3, injury)
                    }
                }
            }
        }
    }
    fn resolve_smash_random_foot(&self, damages: u8) -> IndividualConsequences {
        let affected_side = BodySide::random();
        match self.body().body_part(&BodyPartKind::Leg(affected_side.clone())) {
            None => IndividualConsequences::no_consequences(),
            Some(_) => match self.body().body_part(&BodyPartKind::Foot(affected_side.clone())) {
                None => IndividualConsequences::no_consequences(),
                Some(hand) => {
                    if hand.is_broken() {
                        IndividualConsequences::only_damages(damages + 3)
                    } else {
                        IndividualConsequences::injures(damages + 3, Injury::FootSmashed(affected_side))
                    }
                }
            }
        }
    }
    fn resolve_break_random_arm(&self, damages: u8) -> IndividualConsequences {
        let affected_side = BodySide::random();
        match self.body().body_part(&BodyPartKind::Arm(affected_side.clone())) {
            None => IndividualConsequences::no_consequences(),
            Some(arm) => {
                let injury = match affected_side {
                    BodySide::Right => Injury::RightArmBroken,
                    BodySide::Left => Injury::LeftArmBroken,
                };
                if arm.is_broken() {
                    IndividualConsequences::only_damages(damages + 4)
                } else {
                    IndividualConsequences::injures(damages + 4, injury)
                }
            }
        }
    }
    fn resolve_break_random_leg(&self, damages: u8) -> IndividualConsequences {
        let affected_side = BodySide::random();
        match self.body().body_part(&BodyPartKind::Leg(affected_side.clone())) {
            None => IndividualConsequences::no_consequences(),
            Some(leg) => if leg.is_broken() {
                IndividualConsequences::only_damages(damages + 5)
            } else {
                IndividualConsequences::injures(
                    damages + 5,
                    Injury::OneLegBroken(affected_side),
                )
            }
        }
    }
    fn resolve_wound_genitals(&self, damages: u8) -> IndividualConsequences {
        match self.body().body_part(&BodyPartKind::Genitals) {
            None => IndividualConsequences::no_consequences(),
            Some(_) => IndividualConsequences::damage_on_duration(
                damages + 5,
                DurationDamages::new(),
            ),
        }
    }
    fn resolve_crush_genitals(&self, damages: u8) -> IndividualConsequences {
        match self.body().body_part(&BodyPartKind::Genitals) {
            None => IndividualConsequences::no_consequences(),
            Some(genitals) => {
                if genitals.is_broken() {
                    IndividualConsequences::only_damages(damages + 5)
                } else {
                    IndividualConsequences::injures(damages + 5, Injury::GenitalsCrushed)
                }
            }
        }
    }
    fn resolve_raw_damages(&self, damages: u8) -> IndividualConsequences {
        IndividualConsequences::only_damages(damages)
    }
    fn resolve_duration_damage(&self, damages: u8) -> IndividualConsequences {
        IndividualConsequences::damage_on_duration(damages, DurationDamages::new())
    }
    fn resolve_death(&self) -> IndividualConsequences {
        IndividualConsequences::only_damages(u8::MAX)
    }
    fn resolve_knock_out(&self, damages: u8) -> IndividualConsequences {
        IndividualConsequences::knock_out(damages)
    }
}

pub trait ResolveCriticalHitSelf:
    DealCriticalHit +
    DealDamages +
    ResolveCriticalHit
{
    fn resolve_critical_hit_self(&self) -> IndividualConsequences {
        let critical_hit = self.deal_critical_hit();
        let damages = self.deal_damages();
        let mut consequence = self.resolve_critical_hit(damages, &critical_hit);
        consequence.add_self_critical_hit(critical_hit);
        consequence
    }
}