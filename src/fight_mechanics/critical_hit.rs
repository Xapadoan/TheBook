use crate::dice::Dice;
use crate::equipment::{HasRupture, RuptureTestResult};
use crate::warrior::body::body_part::{BodyPartKind, RandomFunctionalBodyPart};
use crate::warrior::body::body_side::BodySide;
use crate::warrior::protection::{Protectable, RandomProtectedBodyPart};
use crate::warrior::body::injury::{Injury, InjuryKind, MayBeInjured};
use super::fight_action::{ApplyFightActionResult, ShowFightActionResult};
use super::parry::ParryAttemptResult;
use super::{IsUnconscious, RollDamage, TakeDamage};
use crate::warrior::Warrior;
use std::u8::MAX;

pub enum CriticalHitKind {
    DeepIncision,
    ReallyDeepIncision,
    ImpressiveWoundAndArmorDamage,
    PreciseHitAndArmorDamage,
    AccurateHeavyBlowAndArmorDamage,
    PartOfTheArmorIsDestroyed(RuptureTestResult),
    GougedEye,
    SeveredHand,
    SeveredFoot,
    SeveredArm,
    SeveredLeg,
    SeveredGenitals,
    VitalOrganDamage,
    HeartInjury,
    SeriousHeadInjury,
    ImpressiveBruise,
    ImpressiveBruiseAndLimbDislocation,
    RibFacture,
    KneeDislocation,
    BrokenHand,
    SmashedFoot,
    BrokenArm,
    BrokenLeg,
    CrushedGenitals,
    KnockedOut,
    OpenSkullFacture,
    VitalOrganCrushed,
}

pub struct CriticalHitResult {
    kind: CriticalHitKind,
    body_part: Option<BodyPartKind>,
}

impl CriticalHitResult {
    fn new(kind: CriticalHitKind, body_part: Option<BodyPartKind>) -> Self {
        Self { kind, body_part }
    }

    pub fn roll_sharp(victim: &Warrior) -> Self {
        match Dice::D20.roll() {
            1 | 2 => Self::new(CriticalHitKind::DeepIncision, None),
            3 | 4 => Self::new(CriticalHitKind::ReallyDeepIncision, None),
            5 | 6 => Self::new(
                CriticalHitKind::ImpressiveWoundAndArmorDamage,
                Some(victim.body().random_functional_body_part()),
            ),
            7 | 8 => Self::new(
                CriticalHitKind::PreciseHitAndArmorDamage,
                Some(victim.body().random_protected_body_part_fallback_functional()),
            ),
            9 | 10 => Self::new(
                CriticalHitKind::AccurateHeavyBlowAndArmorDamage,
                Some(victim.body().random_protected_body_part_fallback_functional()),
            ),
            11 => match victim.body().random_protected_body_part() {
                Some(part) => Self::new(
                    CriticalHitKind::PartOfTheArmorIsDestroyed(
                        victim.body().body_part(&part).protected_by().unwrap().rupture_test()
                    ),
                    Some(part),
                ),
                None => Self::new(CriticalHitKind::DeepIncision, None),
            },
            12 => Self::new(
                CriticalHitKind::GougedEye,
                Some(BodyPartKind::Eye(BodySide::random()))),
            13 => Self::new(
                CriticalHitKind::SeveredHand,
                Some(BodyPartKind::Hand(BodySide::random())),
            ),
            14 => Self::new(
                CriticalHitKind::SeveredFoot,
                Some(BodyPartKind::Foot(BodySide::random())),
            ),
            15 => Self::new(
                CriticalHitKind::SeveredArm,
                Some(BodyPartKind::Arm(BodySide::random()))
            ),
            16 => Self::new(
                CriticalHitKind::SeveredLeg,
                Some(BodyPartKind::Leg(BodySide::random()))
            ),
            17 => Self::new(
                CriticalHitKind::SeveredGenitals,
                Some(BodyPartKind::Genitals)
            ),
            18 => Self::new(
                CriticalHitKind::VitalOrganDamage,
                Some(BodyPartKind::Torso),
            ),
            19 => Self::new(
                CriticalHitKind::HeartInjury,
                Some(BodyPartKind::Torso),
            ),
            20 => Self::new(
                CriticalHitKind::SeriousHeadInjury,
                Some(BodyPartKind::Head)
            ),
            other => panic!("D20 roll resulted in {other}"),
        }
    }

    pub fn roll_blunt(victim: &Warrior) -> Self {
        match Dice::D20.roll() {
            1 | 2 => Self::new(CriticalHitKind::ImpressiveBruise, None),
            3 | 4 => Self::new(CriticalHitKind::ImpressiveBruiseAndLimbDislocation, None),
            5 | 6 => Self::new(
                CriticalHitKind::RibFacture,
                Some(BodyPartKind::Torso)
            ),
            7 | 8 => Self::new(
                CriticalHitKind::PreciseHitAndArmorDamage,
                Some(victim.body().random_protected_body_part_fallback_functional()),
            ),
            9 | 10 => Self::new(
                CriticalHitKind::AccurateHeavyBlowAndArmorDamage,
                Some(victim.body().random_protected_body_part_fallback_functional())
            ),
            11 => match victim.body().random_protected_body_part() {
                Some(part) => Self::new(
                    CriticalHitKind::PartOfTheArmorIsDestroyed(
                        victim.body().body_part(&part).protected_by().unwrap().rupture_test()
                    ),
                    Some(part),
                ),
                None => Self::new(CriticalHitKind::DeepIncision, None),
            },
            12 => Self::new(
                CriticalHitKind::KneeDislocation,
                Some(BodyPartKind::Knee(BodySide::random())),
            ),
            13 => Self::new(
                CriticalHitKind::BrokenHand,
                Some(BodyPartKind::Hand(BodySide::random())),
            ),
            14 => Self::new(
                CriticalHitKind::SmashedFoot,
                Some(BodyPartKind::Foot(BodySide::random())),
            ),
            15 => Self::new(
                CriticalHitKind::BrokenArm,
                Some(BodyPartKind::Arm(BodySide::random())),
            ),
            16 => Self::new(
                CriticalHitKind::BrokenLeg,
                Some(BodyPartKind::Leg(BodySide::random()))
            ),
            17 => Self::new(
                CriticalHitKind::CrushedGenitals,
                Some(BodyPartKind::Genitals),
            ),
            18 => Self::new(CriticalHitKind::KnockedOut, None),
            19 => Self::new(
                CriticalHitKind::OpenSkullFacture,
                Some(BodyPartKind::Head),
            ),
            20 => Self::new(
                CriticalHitKind::VitalOrganCrushed,
                Some(BodyPartKind::Torso)
            ),
            other => panic!("D20 roll resulted in {other}"),
        }
    }

    fn display_protection_or_limb(&self, victim: &Warrior) -> String {
        let body_part = self.body_part.as_ref().unwrap();
        let precise_target = victim.body().body_part(body_part);
        if precise_target.is_protected() {
            precise_target.protected_by().unwrap().to_string()
        } else {
            precise_target.kind().to_string()
        }
    }

    fn damage_victim_armor(&self, victim: &mut Warrior, damage: u8) {
        let part = victim.body_mut().body_part_mut(self.body_part.as_ref().unwrap());
        let protection = part.protected_by_mut();
        if protection.is_some() {
            protection.unwrap().damage_rupture(damage);
        }
    }
}

impl ShowFightActionResult for CriticalHitResult {
    fn show_fight_action_result(&self, assailant: &Warrior, victim: &Warrior) {
        match &self.kind {
            CriticalHitKind::AccurateHeavyBlowAndArmorDamage => {
                println!(
                    "{} hits {} heavily, damaging his {}",
                    assailant.name(),
                    victim.name(),
                    self.display_protection_or_limb(victim),
                );
            }
            CriticalHitKind::BrokenArm | CriticalHitKind::BrokenHand | CriticalHitKind::BrokenLeg => {
                println!("{} broke {}'s {}", assailant.name(), victim.name(), self.body_part.as_ref().unwrap());
            }
            CriticalHitKind::CrushedGenitals => {
                let genitals = self.body_part.as_ref().unwrap();
                if victim.body().body_part(genitals).is_severed() {
                    println!(
                        "{} smashed the air when {}'s {} should have been",
                        assailant.name(),
                        victim.name(),
                        genitals,
                    )
                } else {
                    println!("{} crushed {}'s genitals", assailant.name(), victim.name());
                }
            }
            CriticalHitKind::DeepIncision => {
                println!("{} cut {} deeply", assailant.name(), victim.name());
            }
            CriticalHitKind::SeveredGenitals => {
                println!("{} severed {}'s genitals", assailant.name(), victim.name());
            }
            CriticalHitKind::GougedEye => {
                let eye = victim.body().body_part(self.body_part.as_ref().unwrap());
                if eye.is_severed() {
                    println!("{}'s {} is already gouged", victim.name(), eye.kind());
                } else {
                    println!("{} gouged {}'s eye", assailant.name(), victim.name());
                }
            }
            CriticalHitKind::HeartInjury => {
                println!("{} pierced {}'s heart", assailant.name(), victim.name());
            }
            CriticalHitKind::ImpressiveBruise => {
                println!("{} bruised {} heavily", assailant.name(), victim.name());
            }
            CriticalHitKind::ImpressiveBruiseAndLimbDislocation => {
                println!(
                    "{} bruised {} heavily, dislocating a limb",
                    assailant.name(),
                    victim.name()
                );
            }
            CriticalHitKind::ImpressiveWoundAndArmorDamage => {
                println!(
                    "{} wounded {} deeply, damaging his {}",
                    assailant.name(),
                    victim.name(),
                    self.display_protection_or_limb(victim),
                );
            }
            CriticalHitKind::KneeDislocation => {
                println!("{} dislocated {}'s knee", assailant.name(), victim.name());
            }
            CriticalHitKind::KnockedOut => {
                println!("{} knocked {} out", assailant.name(), victim.name());
            }
            CriticalHitKind::OpenSkullFacture => {
                println!("{} opened {}'s skull wide", assailant.name(), victim.name());
            }
            CriticalHitKind::PartOfTheArmorIsDestroyed(rupture_test_result) => {
                match rupture_test_result {
                    RuptureTestResult::Fail => println!(
                        "{} destroyed {}'s {}",
                        assailant.name(),
                        victim.name(),
                        self.display_protection_or_limb(victim),
                    ),
                    RuptureTestResult::Success => ParryAttemptResult::Failure.show_fight_action_result(assailant, victim)
                }
            }
            CriticalHitKind::PreciseHitAndArmorDamage => {
                println!(
                    "{} hit {} precisely, damaging his {}",
                    assailant.name(),
                    victim.name(),
                    self.display_protection_or_limb(victim),
                );
            }
            CriticalHitKind::ReallyDeepIncision => {
                println!("{} cut {} really deep", assailant.name(), victim.name());
            }
            CriticalHitKind::RibFacture => {
                println!("{} fractured {}'s rib", assailant.name(), victim.name());
            }
            CriticalHitKind::SeriousHeadInjury => {
                println!("{} cut through {}'s head", assailant.name(), victim.name());
            }
            CriticalHitKind::SeveredArm | CriticalHitKind::SeveredFoot | CriticalHitKind::SeveredHand | CriticalHitKind::SeveredLeg => {
                let body_part = victim.body().body_part(self.body_part.as_ref().unwrap());
                if body_part.is_severed() {
                    println!("{} slashed right where {}'s {} should be", assailant.name(), victim.name(), body_part.kind());
                } else {
                    println!("{} severed {}'s {}", assailant.name(), victim.name(), body_part.kind());
                }
            }
            CriticalHitKind::SmashedFoot => {
                let body_part = victim.body().body_part(self.body_part.as_ref().unwrap());
                if body_part.is_severed() {
                    println!(
                        "{} smashed the ground where {}'s {} should have been",
                        assailant.name(),
                        victim.name(),
                        body_part.kind(),
                    )
                } else {
                    println!("{} smashed {}'s {}", assailant.name(), victim.name(), body_part.kind());
                }
            }
            CriticalHitKind::VitalOrganCrushed => {
                println!(
                    "{} crushed one of {}'s vital organs",
                    assailant.name(),
                    victim.name()
                );
            }
            CriticalHitKind::VitalOrganDamage => {
                println!(
                    "{} damaged one of {}'s vital organs",
                    assailant.name(),
                    victim.name()
                );
            }
        }
    }
}

impl ApplyFightActionResult for CriticalHitResult {
    fn apply_fight_action_result(&self, assailant: &mut Warrior, victim: &mut Warrior) {
        let mut damage = assailant.roll_damage();
        match &self.kind {
            CriticalHitKind::DeepIncision => damage += 1,
            CriticalHitKind::ReallyDeepIncision => damage += 2,
            CriticalHitKind::ImpressiveWoundAndArmorDamage => {
                self.damage_victim_armor(victim, 1);
                damage += 3;
            },
            CriticalHitKind::PreciseHitAndArmorDamage => {
                self.damage_victim_armor(victim, 1);
                damage += 4;
            },
            CriticalHitKind::AccurateHeavyBlowAndArmorDamage => {
                self.damage_victim_armor(victim, 1);
                damage += 5;
            },
            CriticalHitKind::PartOfTheArmorIsDestroyed(rupture_test_result) => {
                let body_part = victim.body_mut().body_part_mut(self.body_part.as_ref().unwrap());
                let protection = body_part.protected_by_mut().unwrap();
                match rupture_test_result {
                    RuptureTestResult::Success => protection.damage_rupture(1),
                    RuptureTestResult::Fail => protection.damage_rupture(MAX),
                }
            },
            CriticalHitKind::GougedEye => {
                let eye = victim.body_mut().body_part_mut(self.body_part.as_ref().unwrap());
                if !eye.is_gouged() {
                    eye.add_injury(Injury::new(
                        InjuryKind::Gouged,
                        -1,
                        -2,
                        format!("{} gouged the {}", assailant.name(), eye.kind()),
                    ));
                }
                damage += 5;
            },
            CriticalHitKind::SeveredHand => 'sever_hand: {
                let target = self.body_part.as_ref().unwrap();
                let arm = match target {
                    BodyPartKind::Hand(side) => match side {
                        BodySide::Left => victim.body().body_part(&BodyPartKind::Arm(BodySide::Left)),
                        BodySide::Right => victim.body().body_part(&BodyPartKind::Arm(BodySide::Right)),
                    }
                    other => panic!("Cannot target a {other} to sever a hand"),
                };
                if arm.is_severed() { break 'sever_hand; }
                let hand = victim.body_mut().body_part_mut(target);
                if hand.is_severed() { break 'sever_hand; }
                let injury_reason = format!("{} severed the {}", assailant.name(), hand.kind());
                let injury = match hand.kind() {
                    BodyPartKind::Hand(side) => match side {
                        BodySide::Left => Injury::new(
                            InjuryKind::Severed,
                            -2,
                            -3,
                            injury_reason,
                        ),
                        BodySide::Right => Injury::new(
                            InjuryKind::Severed,
                            -5,
                            -6,
                            injury_reason,
                        ),
                    },
                    other => panic!("{other} can't be severed the same way a hand does")
                };
                hand.add_injury(injury);
                damage += 6;
            },
            CriticalHitKind::SeveredFoot => 'sever_foot: {
                let target = self.body_part.as_ref().unwrap();
                let leg = match target {
                    BodyPartKind::Foot(side) => match side {
                        BodySide::Left => victim.body().body_part(&BodyPartKind::Leg(BodySide::Left)),
                        BodySide::Right => victim.body().body_part(&BodyPartKind::Leg(BodySide::Right)),
                    }
                    other => panic!("Cannot target a {other} to sever a hand"),
                };
                if leg.is_severed() { break 'sever_foot; }
                let foot = victim.body_mut().body_part_mut(target);
                if foot.is_severed() { break 'sever_foot; }
                foot.add_injury(Injury::new(
                    InjuryKind::Severed,
                    -2,
                    -2,
                    format!("{} severed the {}", assailant.name(), foot.kind()),
                ));
                damage += 6;
            },
            CriticalHitKind::SeveredArm => 'sever_arm: {
                let arm = victim.body_mut().body_part_mut(self.body_part.as_ref().unwrap());
                if arm.is_severed() { break 'sever_arm; }
                let injury_reason = format!("{} severed the {}", assailant.name(), arm.kind());
                let injury = match arm.kind() {
                    BodyPartKind::Arm(side) => match side {
                        BodySide::Left => Injury::new(
                            InjuryKind::Severed,
                            -3,
                            -4,
                            injury_reason,
                        ),
                        BodySide::Right => Injury::new(
                            InjuryKind::Severed,
                            -5,
                            -6,
                            injury_reason,
                        ),
                    }
                    other => panic!("{other} can't be severed the same way an arm does")
                };
                arm.add_injury(injury);
                damage += 7;
            },
            CriticalHitKind::SeveredLeg => 'sever_leg: {
                let leg = victim.body_mut().body_part_mut(self.body_part.as_ref().unwrap());
                if !leg.is_severed() { break 'sever_leg; }
                leg.add_injury(Injury::new(
                    InjuryKind::Severed,
                    -4,
                    -6,
                    format!("{} is severed", leg.kind()),
                ));
                damage += 8;
            },
            CriticalHitKind::SeveredGenitals => 'sever_genitals: {
                let genitals = victim.body_mut().body_part_mut(self.body_part.as_ref().unwrap());
                if genitals.is_severed() { break 'sever_genitals }
                genitals.add_injury(Injury::new(
                    InjuryKind::Severed,
                    0,
                    0,
                    format!("{} were severed", genitals.kind()),
                ));
                damage += 5;
            },
            CriticalHitKind::VitalOrganDamage => {
                println!("[WARN] duration damage is not implemented");
                damage += 9;
            },
            CriticalHitKind::HeartInjury => damage = MAX,
            CriticalHitKind::SeriousHeadInjury => damage = MAX,
            CriticalHitKind::ImpressiveBruise => damage += 1,
            CriticalHitKind::ImpressiveBruiseAndLimbDislocation => damage += 2,
            CriticalHitKind::RibFacture => damage += 3,
            CriticalHitKind::KneeDislocation => 'dislocate_knee: {
                let target = self.body_part.as_ref().unwrap();
                let leg = match target {
                    BodyPartKind::Knee(side) => match side {
                        BodySide::Left => victim.body().body_part(&BodyPartKind::Leg(BodySide::Left)),
                        BodySide::Right => victim.body().body_part(&BodyPartKind::Leg(BodySide::Right)),                        
                    },
                    other => panic!("Can't target a {other} to dislocate a knee"),
                };
                if leg.is_severed() || leg.is_broken() { break 'dislocate_knee; }
                let knee = victim.body_mut().body_part_mut(target);
                knee.add_injury(Injury::new(
                    InjuryKind::Dislocated,
                    -1,
                    -2,
                    format!("{} dislocated the {}", assailant.name(), knee.kind())
                ));
                damage += 3;
            },
            CriticalHitKind::BrokenHand => 'break_hand: {
                let target = self.body_part.as_ref().unwrap();
                let arm = match target {
                    BodyPartKind::Hand(side) => match side {
                        BodySide::Left => victim.body().body_part(&BodyPartKind::Arm(BodySide::Left)),
                        BodySide::Right => victim.body().body_part(&BodyPartKind::Arm(BodySide::Right)),
                    },
                    other => panic!("Can't target {other} to sever a hand"),
                };
                if arm.is_severed() { break 'break_hand; }
                let hand = victim.body_mut().body_part_mut(target);
                if hand.is_severed() || hand.is_broken() { break 'break_hand; }
                let injury_reason = format!("{} is broken", hand.kind());
                let injury = match hand.kind() {
                    BodyPartKind::Hand(side) => match side {
                        BodySide::Left => Injury::new(
                            InjuryKind::Broken,
                            -2,
                            -3,
                            injury_reason,
                        ),
                        BodySide::Right => Injury::new(
                            InjuryKind::Broken,
                            -5,
                            -6,
                            injury_reason,
                        )
                    },
                    other => panic!("{other} cannot be broken as a hand does")
                };
                hand.add_injury(injury);
                damage += 3;
            },
            CriticalHitKind::SmashedFoot => 'smash_foot: {
                let target = self.body_part.as_ref().unwrap();
                let leg = match target {
                    BodyPartKind::Foot(side) => match side {
                        BodySide::Left => victim.body().body_part(&&BodyPartKind::Leg(BodySide::Left)),
                        BodySide::Right => victim.body().body_part(&&BodyPartKind::Leg(BodySide::Right)),
                    },
                    other => panic!("Can't target {other} to sever a foot"),
                };
                if leg.is_severed() { break 'smash_foot; }
                let foot = victim.body_mut().body_part_mut(target);
                if foot.is_severed() || foot.is_broken() { break 'smash_foot; }
                foot.add_injury(Injury::new(
                    InjuryKind::Broken,
                    -2,
                    -2,
                    format!("{} is broken", foot.kind()),
                ));
                damage += 3;
            },
            CriticalHitKind::BrokenArm => 'break_arm: {
                let arm = victim.body_mut().body_part_mut(self.body_part.as_ref().unwrap());
                if arm.is_severed() { break 'break_arm; }
                let injury_reason = format!("{} is broken", arm.kind());
                let injury = match arm.kind() {
                    BodyPartKind::Arm(side) => match side {
                        BodySide::Left => Injury::new(
                            InjuryKind::Broken,
                            -2,
                            -3,
                            injury_reason,
                        ),
                        BodySide::Right => Injury::new(
                            InjuryKind::Broken,
                            -5,
                            -6,
                            injury_reason,
                        ),
                    },
                    other => panic!("{other} cannot be broken same as an arm does"),
                };
                arm.add_injury(injury);
                damage += 4;
            },
            CriticalHitKind::BrokenLeg => 'break_leg: {
                let leg = victim.body_mut().body_part_mut(self.body_part.as_ref().unwrap());
                if leg.is_severed() { break 'break_leg; }
                leg.add_injury(Injury::new(
                    InjuryKind::Broken,
                    -4,
                    -6,
                    format!("{} is broken", leg.kind())
                ));
                damage += 5;
            },
            CriticalHitKind::CrushedGenitals => 'crush_genitals: {
                let genitals = victim.body_mut().body_part_mut(self.body_part.as_ref().unwrap());
                if genitals.is_severed() { break 'crush_genitals; }
                genitals.add_injury(Injury::new(
                    InjuryKind::Broken,
                    0,
                    0,
                    format!("{} are crushed", genitals.kind())
                ));
                damage += 5;
            },
            CriticalHitKind::KnockedOut => victim.set_unconscious(),
            CriticalHitKind::OpenSkullFacture => damage = MAX,
            CriticalHitKind::VitalOrganCrushed => damage = MAX,
        }
        victim.take_damage(damage);
    }
}
