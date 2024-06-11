use crate::dice::Dice;
use crate::warrior::body::body_part::{BodyPartKind, RandomFunctionalBodyPart};
use crate::warrior::body::body_side::BodySide;
use crate::warrior::protection::Protectable;
use crate::warrior::body::injury::{Injury, MayBeInjured};
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
    PartOfTheArmorIsDestroyed,
    GougedEye,
    SeveredHand,
    SeveredFoot,
    SeveredArm,
    SeveredLeg,
    GenitalsDamage,
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
            11 => Self::new(
                CriticalHitKind::PartOfTheArmorIsDestroyed,
                Some(victim.body().random_protected_body_part_fallback_functional()),
            ),
            12 => Self::new(CriticalHitKind::GougedEye, None),
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
                Some(BodyPartKind::Leg(BodySide::random()))
            ),
            16 => Self::new(
                CriticalHitKind::SeveredLeg,
                Some(BodyPartKind::Leg(BodySide::random()))
            ),
            17 => Self::new(CriticalHitKind::GenitalsDamage, None),
            18 => Self::new(CriticalHitKind::VitalOrganDamage, None),
            19 => Self::new(CriticalHitKind::HeartInjury, None),
            20 => Self::new(CriticalHitKind::SeriousHeadInjury, None),
            other => panic!("D20 roll resulted in {other}"),
        }
    }

    pub fn roll_blunt(victim: &Warrior) -> Self {
        match Dice::D20.roll() {
            1 | 2 => Self::new(CriticalHitKind::ImpressiveBruise, None),
            3 | 4 => Self::new(CriticalHitKind::ImpressiveBruiseAndLimbDislocation, None),
            5 | 6 => Self::new(CriticalHitKind::RibFacture, None),
            7 | 8 => Self::new(
                CriticalHitKind::PreciseHitAndArmorDamage,
                Some(victim.body().random_protected_body_part_fallback_functional()),
            ),
            9 | 10 => Self::new(
                CriticalHitKind::AccurateHeavyBlowAndArmorDamage,
                Some(victim.body().random_protected_body_part_fallback_functional())
            ),
            11 => Self::new(
                CriticalHitKind::PartOfTheArmorIsDestroyed,
                Some(victim.body().random_protected_body_part_fallback_functional())
            ),
            12 => Self::new(
                CriticalHitKind::KneeDislocation,
                Some(BodyPartKind::Leg(BodySide::random())),
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
            17 => Self::new(CriticalHitKind::CrushedGenitals, None),
            18 => Self::new(CriticalHitKind::KnockedOut, None),
            19 => Self::new(CriticalHitKind::OpenSkullFacture, None),
            20 => Self::new(CriticalHitKind::VitalOrganCrushed, None),
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

    fn damage_victim_armor(&self, victim: &mut Warrior) {
        let part = victim.body_mut().body_part_mut(self.body_part.as_ref().unwrap());
        let protection = part.protected_by_mut();
        if protection.is_some() {
            protection.unwrap().take_damage(1);
        }
    }
}

impl ShowFightActionResult for CriticalHitResult {
    fn show_fight_action_result(&self, assailant: &Warrior, victim: &Warrior) {
        match self.kind {
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
                println!("{} crushed {}'s genitals", assailant.name(), victim.name());
            }
            CriticalHitKind::DeepIncision => {
                println!("{} cut {} deeply", assailant.name(), victim.name());
            }
            CriticalHitKind::GenitalsDamage => {
                println!("{} hit {}'s genitals", assailant.name(), victim.name());
            }
            CriticalHitKind::GougedEye => {
                println!("{} gouged {}'s eye", assailant.name(), victim.name());
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
            CriticalHitKind::PartOfTheArmorIsDestroyed => {
                let body_part = self.body_part.as_ref().unwrap();
                let precise_target = victim.body().body_part(body_part);
                if precise_target.is_protected() {
                    println!(
                        "{} destroyed {}'s {}",
                        assailant.name(),
                        victim.name(),
                        self.display_protection_or_limb(victim),
                    )
                } else {
                    ParryAttemptResult::Failure.show_fight_action_result(assailant, victim)
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
                println!("{} smashed {}'s {}", assailant.name(), victim.name(), self.body_part.as_ref().unwrap());
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
        match self.kind {
            CriticalHitKind::DeepIncision => damage += 1,
            CriticalHitKind::ReallyDeepIncision => damage += 2,
            CriticalHitKind::ImpressiveWoundAndArmorDamage => {
                self.damage_victim_armor(victim);
                damage += 3;
            },
            CriticalHitKind::PreciseHitAndArmorDamage => {
                self.damage_victim_armor(victim);
                damage += 4;
            },
            CriticalHitKind::AccurateHeavyBlowAndArmorDamage => {
                self.damage_victim_armor(victim);
                damage += 5;
            },
            CriticalHitKind::PartOfTheArmorIsDestroyed => {
                self.damage_victim_armor(victim);
            },
            CriticalHitKind::GougedEye => {
                println!("[WARN] deep wounds not implemented");
                damage += 5;
            },
            CriticalHitKind::SeveredHand | CriticalHitKind::SeveredFoot => {
                let body_part = victim.body_mut().body_part_mut(self.body_part.as_ref().unwrap());
                if !body_part.is_severed() {
                    body_part.sever();
                    let injury_reason = format!("{} severed the {}", assailant.name(), body_part.kind());
                    let injury = match body_part.kind() {
                        BodyPartKind::Hand(side) => {
                            match side {
                                BodySide::Left => Injury::new(-2, -3, injury_reason),
                                BodySide::Right => Injury::new(-5, -6, injury_reason),
                            }
                        }
                        BodyPartKind::Foot(_) => Injury::new(-2, -2, injury_reason),
                        other => panic!("{other} can't be severed the same way a foot or hand does")
                    };
                    body_part.add_injury(injury);
                    damage += 6;
                }
            },
            CriticalHitKind::SeveredArm => {
                let arm = victim.body_mut().body_part_mut(self.body_part.as_ref().unwrap());
                if !arm.is_severed() {
                    arm.sever();
                    let injury_reason = format!("{} severed the {}", assailant.name(), arm.kind());
                    let injury = match arm.kind() {
                        BodyPartKind::Arm(side) => {
                            match side {
                                BodySide::Left => Injury::new(-3, -4, injury_reason),
                                BodySide::Right => Injury::new(-5, -6, injury_reason),
                            }
                        },
                        other => panic!("{other} can't be severed the same way an arm does")
                    };
                    arm.add_injury(injury);
                    damage += 7;
                }
            },
            CriticalHitKind::SeveredLeg => {
                let leg = victim.body_mut().body_part_mut(self.body_part.as_ref().unwrap());
                if !leg.is_severed() {
                    leg.sever();
                    let injury_reason = format!("{} severed the {}", assailant.name(), leg.kind());
                    let injury = match leg.kind() {
                        BodyPartKind::Leg(side) => {
                            match side {
                                BodySide::Left => Injury::new(-3, -4, injury_reason),
                                BodySide::Right => Injury::new(-5, -6, injury_reason),
                            }
                        },
                        other => panic!("{other} can't be severed the same way an arm does")
                    };
                    leg.add_injury(injury);
                    damage += 8;
                }
            },
            CriticalHitKind::GenitalsDamage => {
                println!("[WARN] duration damage is not implemented");
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
            CriticalHitKind::KneeDislocation => {
                println!("[WARN] deep wounds not implemented");
                damage += 3;
            },
            CriticalHitKind::BrokenHand => {
                println!("[WARN] deep wounds not implemented");
                damage += 3;
            },
            CriticalHitKind::SmashedFoot => {
                println!("[WARN] deep wounds not implemented");
                damage += 3;
            },
            CriticalHitKind::BrokenArm => {
                println!("[WARN] deep wounds not implemented");
                damage += 4;
            },
            CriticalHitKind::BrokenLeg => {
                println!("[WARN] deep wounds not implemented");
                damage += 5;
            },
            CriticalHitKind::CrushedGenitals => {
                println!("[WARN] deep wounds not implemented");
                damage += 5;
            },
            CriticalHitKind::KnockedOut => victim.set_unconscious(),
            CriticalHitKind::OpenSkullFacture => damage = MAX,
            CriticalHitKind::VitalOrganCrushed => damage = MAX,
        }
        victim.take_damage(damage);
    }
}
