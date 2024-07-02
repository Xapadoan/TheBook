use crate::dice::Dice;
use crate::equipment::{HasRupture, MayHaveRuptureDamage, MayHaveTestedRupture, RuptureTestResult};
use crate::gen_random::GenRandom;
use crate::modifiers::{ApplyDamageModifier, Modifier};
use crate::warrior::assault::damage_summary::DamageSummary;
use crate::warrior::assault::execute_action::ExecuteAction;
use crate::warrior::assault::parry::parry_attempt::ParryThreshold;
use crate::warrior::assault::show_action::ShowAction;
use crate::warrior::assault::Assault;
use crate::warrior::body::body_part::{BodyPartKind, RandomFunctionalBodyPart, MayTargetBodyPart};
use crate::warrior::body::body_side::BodySide;
use crate::warrior::body::{HasBody, HasMutableBody};
use crate::warrior::duration_damage::MayHaveDurationDamage;
use crate::warrior::protection::{Protectable, RandomProtectedBodyPart};
use crate::warrior::body::injury::{Injury, InjuryKind, MayBeInjured, MayCauseInjury, TakeInjury};
use crate::warrior::weapon::{MayHaveMutableWeapon, MayHaveWeapon, TakeWeapon};
use crate::warrior::{IsUnconscious, HasName, RollDamage, TakeDamage, TakeReducedDamage};
use crate::warrior::temporary_handicap::parries_miss::CanMissParries;
use crate::warrior::temporary_handicap::assaults_miss::CanMissAssaults;

use super::can_be_attacked::CanBeAttacked;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct CriticalHitResult {
    kind: CriticalHitKind,
    body_part_kind: Option<BodyPartKind>,
    rupture_test_result: Option<RuptureTestResult>,
    rupture_damage: Option<u8>,
    dmg_modifier: Modifier,
    injury: Option<Injury>,
}

impl CriticalHitResult {
    fn new<V: HasBody>(victim: &V, kind: CriticalHitKind) -> Self {
        match kind {
            CriticalHitKind::BrokenArm |
            CriticalHitKind::BrokenLeg => {
                let affected_side = BodySide::gen_random();
                let body_part_kind = match kind {
                    CriticalHitKind::BrokenArm => BodyPartKind::Arm(affected_side),
                    CriticalHitKind::BrokenLeg => BodyPartKind::Leg(affected_side),
                    _ => panic!("Match should not be possible")
                };
                let body_part = victim.body().body_part(&body_part_kind);
                if body_part.is_severed() {
                    return Self {
                        kind,
                        body_part_kind: Some(body_part_kind),
                        rupture_test_result: None,
                        rupture_damage: None,
                        dmg_modifier: Modifier::new(0),
                        injury: None,
                    };
                } else {
                    let (attack, parry, additional_damage) = match &body_part_kind {
                        BodyPartKind::Arm(side) => match side {
                            BodySide::Left => (-2, -3, 4),
                            BodySide::Right => (-5, -6, 4),
                        },
                        BodyPartKind::Leg(_) => (-4, -6, 5),
                        _ => panic!("Match should not be possible")
                    };
                    Self {
                        kind,
                        body_part_kind: Some(body_part_kind),
                        rupture_test_result: None,
                        rupture_damage: None,
                        dmg_modifier: Modifier::new(additional_damage),
                        injury: Some(Injury::new(
                            InjuryKind::Broken,
                            attack,
                            parry,
                            // format!("{} is broken", body_part.kind()),
                        ))
                    }
                }
            },
            CriticalHitKind::BrokenHand |
            CriticalHitKind::SmashedFoot |
            CriticalHitKind::KneeDislocation => {
                let affected_side = BodySide::gen_random();
                let parent_affected_side = affected_side.clone();
                let body_part_kind = match kind {
                    CriticalHitKind::BrokenHand => BodyPartKind::Hand(affected_side),
                    CriticalHitKind::SmashedFoot => BodyPartKind::Foot(affected_side),
                    CriticalHitKind::KneeDislocation => BodyPartKind::Knee(affected_side),
                    _ => panic!("Match should not be possible")
                };
                let parent_body_part_kind = match kind {
                    CriticalHitKind::BrokenHand => BodyPartKind::Arm(parent_affected_side),
                    CriticalHitKind::SmashedFoot |
                    CriticalHitKind::KneeDislocation => BodyPartKind::Leg(parent_affected_side),
                    _ => panic!("Match should not be possible")
                };
                let parent_body_part = victim.body().body_part(&parent_body_part_kind);
                let body_part = victim.body().body_part(&body_part_kind);
                if parent_body_part.is_severed() | body_part.is_severed() {
                    return Self {
                        kind,
                        body_part_kind: Some(body_part_kind),
                        rupture_test_result: None,
                        rupture_damage: None,
                        dmg_modifier: Modifier::new(0),
                        injury: None,
                    };
                } else  {
                    let (injury_kind, attack, parry) = match &body_part_kind {
                        BodyPartKind::Hand(side) => match side {
                            BodySide::Left => (InjuryKind::Broken, -2, -3),
                            BodySide::Right => (InjuryKind::Broken, -5, -6),
                        },
                        BodyPartKind::Foot(_) => (InjuryKind::Broken, -2, -2),
                        BodyPartKind::Knee(_) => (InjuryKind::Dislocated, -1, -2),
                        _ => panic!("Match should not be possible")
                    };
                    
                    Self {
                        kind,
                        body_part_kind: Some(body_part_kind),
                        rupture_test_result: None,
                        rupture_damage: None,
                        dmg_modifier: Modifier::new(3),
                        injury: Some(Injury::new(
                            injury_kind,
                            attack,
                            parry,
                            // format!("{} is broken", body_part.kind()),
                        ))
                    }
                }
            },
            CriticalHitKind::CrushedGenitals => {
                let body_part_kind = BodyPartKind::Genitals;
                let genitals = victim.body().body_part(&body_part_kind);
                if genitals.is_severed() {
                    Self {
                        kind,
                        body_part_kind: Some(body_part_kind),
                        rupture_test_result: None,
                        rupture_damage: None,
                        dmg_modifier: Modifier::new(0),
                        injury: None,
                    }
                } else {
                    Self {
                        kind,
                        body_part_kind: Some(body_part_kind),
                        rupture_test_result: None,
                        rupture_damage: None,
                        dmg_modifier: Modifier::new(5),
                        injury: Some(Injury::new(
                            InjuryKind::Broken,
                            0,
                            0,
                            // format!("{} were crushed", genitals.kind())
                        ))
                    }
                }
            },
            CriticalHitKind::DeepIncision |
            CriticalHitKind::ReallyDeepIncision |
            CriticalHitKind::ImpressiveBruise |
            CriticalHitKind::ImpressiveBruiseAndLimbDislocation |
            CriticalHitKind::RibFacture => {
                let additional_damage = match kind {
                    CriticalHitKind::DeepIncision |
                    CriticalHitKind::ImpressiveBruise => 1,
                    CriticalHitKind::ReallyDeepIncision |
                    CriticalHitKind::ImpressiveBruiseAndLimbDislocation => 2,
                    CriticalHitKind::RibFacture => 3,
                    _ => panic!("Match should not be possible")
                };
                Self {
                    kind,
                    body_part_kind: None,
                    rupture_test_result: None,
                        rupture_damage: None,
                    dmg_modifier: Modifier::new(additional_damage),
                    injury: None,
                }
            },
            CriticalHitKind::GougedEye => {
                let body_part_kind = BodyPartKind::Eye(BodySide::gen_random());
                let body_part = victim.body().body_part(&body_part_kind);
                let injury = if body_part.is_gouged() {
                    None
                } else {
                    Some(Injury::new(
                        InjuryKind::Gouged,
                        -1,
                        -2,
                        // format!("{} is gouged", body_part_kind)
                    ))
                };
                Self {
                    kind,
                    body_part_kind: Some(body_part_kind),
                    rupture_test_result: None,
                    rupture_damage: None,
                    dmg_modifier: Modifier::new(5),
                    injury
                }
            },
            CriticalHitKind::HeartInjury |
            CriticalHitKind::SeriousHeadInjury |
            CriticalHitKind::OpenSkullFacture |
            CriticalHitKind::VitalOrganCrushed => {
                let body_part_kind = match kind {
                    CriticalHitKind::HeartInjury |
                    CriticalHitKind::VitalOrganCrushed => BodyPartKind::Torso,
                    CriticalHitKind::SeriousHeadInjury |
                    CriticalHitKind::OpenSkullFacture => BodyPartKind::Head,
                    _ => panic!("Match should not be possible"),
                };
                let body_part = victim.body().body_part(&body_part_kind);
                match body_part.protected_by() {
                    Some(protection) => {
                        let rupture_test_result = protection.rupture_test();
                        let additional_damage = match rupture_test_result {
                            RuptureTestResult::Fail => i8::MAX,
                            RuptureTestResult::Success => 0,
                        };
                        Self {
                            kind,
                            body_part_kind: Some(body_part_kind),
                            rupture_test_result: Some(rupture_test_result),
                            rupture_damage: Some(u8::MAX),
                            dmg_modifier: Modifier::new(additional_damage),
                            injury: None,
                        }
                    },
                    None => Self {
                        kind,
                        body_part_kind: Some(body_part_kind),
                        rupture_test_result: None,
                        rupture_damage: None,
                        dmg_modifier: Modifier::new(i8::MAX),
                        injury: None,
                    }
                }
            },
            CriticalHitKind::ImpressiveWoundAndArmorDamage |
            CriticalHitKind::PreciseHitAndArmorDamage |
            CriticalHitKind::AccurateHeavyBlowAndArmorDamage |
            CriticalHitKind::PartOfTheArmorIsDestroyed => {
                let protected_body_part_kind = victim.body().random_protected_body_part();
                let additional_damage = match kind {
                    CriticalHitKind::ImpressiveWoundAndArmorDamage => 1,
                    CriticalHitKind::PreciseHitAndArmorDamage => 2,
                    CriticalHitKind::AccurateHeavyBlowAndArmorDamage => 3,
                    CriticalHitKind::PartOfTheArmorIsDestroyed => 0,
                    _ => panic!("Match should not be possible"),
                };
                let (body_part_kind, rupture_damage) = match protected_body_part_kind {
                    Some(body_part_kind) => {
                        let rupture_damage = match kind {
                            CriticalHitKind::ImpressiveWoundAndArmorDamage => 1,
                            CriticalHitKind::PreciseHitAndArmorDamage => 2,
                            CriticalHitKind::AccurateHeavyBlowAndArmorDamage => 3,
                            CriticalHitKind::PartOfTheArmorIsDestroyed => u8::MAX,
                            _ => panic!("Match should not be possible"),
                        };
                        (Some(body_part_kind), Some(rupture_damage))
                    },
                    None => (Some(victim.body().random_functional_body_part()), None),
                };
                Self {
                    kind,
                    body_part_kind,
                    rupture_test_result: None,
                    rupture_damage,
                    dmg_modifier: Modifier::new(additional_damage),
                    injury: None,
                }
            },
            CriticalHitKind::KnockedOut => Self {
                kind,
                body_part_kind: None,
                rupture_test_result: None,
                rupture_damage: None,
                dmg_modifier: Modifier::new(0),
                injury: None
            },
            CriticalHitKind::SeveredArm |
            CriticalHitKind::SeveredLeg => {
                let affected_side = BodySide::gen_random();
                let body_part_kind = match kind {
                    CriticalHitKind::SeveredArm => BodyPartKind::Arm(affected_side),
                    CriticalHitKind::SeveredLeg => BodyPartKind::Leg(affected_side),
                    _ => panic!("Match should not be possible"),
                };
                let body_part = victim.body().body_part(&body_part_kind);
                if body_part.is_severed() {
                    return Self {
                        kind,
                        body_part_kind: Some(body_part_kind),
                        rupture_test_result: None,
                        rupture_damage: None,
                        dmg_modifier: Modifier::new(0),
                        injury: None,
                    };
                }
                let additional_damages = match kind {
                    CriticalHitKind::SeveredArm => 7,
                    CriticalHitKind::SeveredLeg => 8,
                    _ => panic!("Match should not be possible"),
                };
                let (attack, parry) = match &body_part_kind {
                    BodyPartKind::Arm(side) => match side {
                        BodySide::Left => (-3, -4),
                        BodySide::Right => (-5, -6),
                    },
                    BodyPartKind::Leg(_) => (-4, -6),
                    _ => panic!("Match should not be possible"),
                };
                let injury = Injury::new(
                    InjuryKind::Severed,
                    attack,
                    parry,
                    // format!("{} is severed", body_part.kind())
                );
                match body_part.protected_by() {
                    Some(protection) => {
                        let rupture_test_result = protection.rupture_test();
                        let (injury, rupture_damage) = match rupture_test_result {
                            RuptureTestResult::Fail => (Some(injury), u8::MAX),
                            RuptureTestResult::Success => (None, 2)
                        };
                        Self {
                            kind,
                            body_part_kind: Some(body_part_kind),
                            rupture_test_result: Some(rupture_test_result),
                            rupture_damage: Some(rupture_damage),
                            dmg_modifier: Modifier::new(additional_damages),
                            injury
                        }
                    },
                    None => Self {
                        kind,
                        body_part_kind: Some(body_part_kind),
                        rupture_test_result: None,
                        rupture_damage: None,
                        dmg_modifier: Modifier::new(additional_damages),
                        injury: Some(injury),
                    }
                }
            },
            CriticalHitKind::SeveredFoot |
            CriticalHitKind::SeveredHand => {
                let affected_side = BodySide::gen_random();
                let parent_affected_side = affected_side.clone();
                let body_part_kind = match kind {
                    CriticalHitKind::SeveredHand => BodyPartKind::Hand(affected_side),
                    CriticalHitKind::SeveredFoot => BodyPartKind::Foot(affected_side),
                    _ => panic!("Match should not be possible")
                };
                let parent_body_part_kind = match kind {
                    CriticalHitKind::SeveredHand => BodyPartKind::Arm(parent_affected_side),
                    CriticalHitKind::SeveredFoot => BodyPartKind::Leg(parent_affected_side),
                    _ => panic!("Match should not be possible")
                };
                let parent_body_part = victim.body().body_part(&parent_body_part_kind);
                let body_part = victim.body().body_part(&body_part_kind);
                if parent_body_part.is_severed() | body_part.is_severed() {
                    return Self {
                        kind,
                        body_part_kind: Some(body_part_kind),
                        rupture_test_result: None,
                        rupture_damage: None,
                        dmg_modifier: Modifier::new(0),
                        injury: None,
                    };
                }
                let (attack, parry) = match &body_part_kind {
                    BodyPartKind::Hand(side) => match side {
                        BodySide::Left => (-2, -3),
                        BodySide::Right => (-5, -6),
                    },
                    BodyPartKind::Foot(_) => (-2, -2),
                    _ => panic!("Match should not be possible"),
                };
                let injury = Injury::new(
                    InjuryKind::Severed,
                    attack,
                    parry,
                    // format!("{} is severed", body_part.kind())
                );
                match body_part.protected_by() {
                    Some(protection) => {
                        let rupture_test_result = protection.rupture_test();
                        let (injury, rupture_damage) = match rupture_test_result {
                            RuptureTestResult::Fail => (Some(injury), u8::MAX),
                            RuptureTestResult::Success => (None, 2)
                        };
                        Self {
                            kind,
                            body_part_kind: Some(body_part_kind),
                            rupture_test_result: Some(rupture_test_result),
                            rupture_damage: Some(rupture_damage),
                            dmg_modifier: Modifier::new(6),
                            injury
                        }
                    },
                    None => Self {
                        kind,
                        body_part_kind: Some(body_part_kind),
                        rupture_test_result: None,
                        rupture_damage: None,
                        dmg_modifier: Modifier::new(6),
                        injury: Some(injury),
                    }
                }
            },
            CriticalHitKind::SeveredGenitals |
            CriticalHitKind::VitalOrganDamage => {
                let body_part_kind = BodyPartKind::Torso;
                let body_part = victim.body().body_part(&body_part_kind);
                let additional_damage = match kind {
                    CriticalHitKind::SeveredGenitals => 5,
                    CriticalHitKind::VitalOrganDamage => 9,
                    _ => panic!("Match should not be possible"),
                };
                let (rupture_damage, rupture_test_result) = match body_part.protected_by() {
                    Some(protection) => {
                        let rupture_test_result = protection.rupture_test();
                        match rupture_test_result {
                            RuptureTestResult::Fail => (Some(u8::MAX), Some(rupture_test_result)),
                            RuptureTestResult::Success => (Some(2), Some(rupture_test_result)),
                        }
                    },
                    None => (None, None),
                };
                Self {
                    kind,
                    body_part_kind: Some(body_part_kind),
                    rupture_test_result,
                    rupture_damage,
                    dmg_modifier: Modifier::new(additional_damage),
                    injury: None,
                }
            }
        }
    }

    pub fn roll_sharp<V: HasBody>(victim: &V) -> Self {
        match Dice::D20.roll() {
            1 | 2 => Self::new(victim, CriticalHitKind::DeepIncision),
            3 | 4 => Self::new(victim, CriticalHitKind::ReallyDeepIncision),
            5 | 6 => Self::new(victim, CriticalHitKind::ImpressiveWoundAndArmorDamage),
            7 | 8 => Self::new(victim, CriticalHitKind::PreciseHitAndArmorDamage),
            9 | 10 => Self::new(victim, CriticalHitKind::AccurateHeavyBlowAndArmorDamage),
            11 => Self::new(victim, CriticalHitKind::PartOfTheArmorIsDestroyed),
            12 => Self::new(victim, CriticalHitKind::GougedEye),
            13 => Self::new(victim, CriticalHitKind::SeveredHand),
            14 => Self::new(victim, CriticalHitKind::SeveredFoot),
            15 => Self::new(victim, CriticalHitKind::SeveredArm),
            16 => Self::new(victim, CriticalHitKind::SeveredLeg),
            17 => Self::new(victim, CriticalHitKind::SeveredGenitals),
            18 => Self::new(victim, CriticalHitKind::VitalOrganDamage),
            19 => Self::new(victim, CriticalHitKind::HeartInjury),
            20 => Self::new(victim, CriticalHitKind::SeriousHeadInjury),
            other => panic!("D20 roll resulted in {other}"),
        }
    }

    pub fn roll_blunt<V: HasBody>(victim: &V) -> Self {
        match Dice::D20.roll() {
            1 | 2 => Self::new(victim, CriticalHitKind::ImpressiveBruise),
            3 | 4 => Self::new(victim, CriticalHitKind::ImpressiveBruiseAndLimbDislocation),
            5 | 6 => Self::new(victim, CriticalHitKind::RibFacture),
            7 | 8 => Self::new(victim, CriticalHitKind::PreciseHitAndArmorDamage),
            9 | 10 => Self::new(victim, CriticalHitKind::AccurateHeavyBlowAndArmorDamage),
            11 => Self::new(victim, CriticalHitKind::PartOfTheArmorIsDestroyed),
            12 => Self::new(victim, CriticalHitKind::KneeDislocation),
            13 => Self::new(victim, CriticalHitKind::BrokenHand),
            14 => Self::new(victim, CriticalHitKind::SmashedFoot),
            15 => Self::new(victim, CriticalHitKind::BrokenArm),
            16 => Self::new(victim, CriticalHitKind::BrokenLeg),
            17 => Self::new(victim, CriticalHitKind::CrushedGenitals),
            18 => Self::new(victim, CriticalHitKind::KnockedOut),
            19 => Self::new(victim, CriticalHitKind::OpenSkullFacture),
            20 => Self::new(victim, CriticalHitKind::VitalOrganCrushed),
            other => panic!("D20 roll resulted in {other}"),
        }
    }

    pub fn display_protection_or_limb<V: HasBody>(&self, victim: &V) -> String {
        let body_part = victim.body().body_part(self.body_part_kind.as_ref().unwrap());
        match body_part.protected_by() {
            Some(protection) => protection.to_string(),
            None => body_part.kind().to_string()
        }
    }

    pub fn kind(&self) -> &CriticalHitKind {
        &self.kind
    }

    pub fn self_inflict<T>(&mut self, victim: &mut T) -> DamageSummary
    where
        T: RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + HasName + HasMutableBody + TakeDamage + TakeReducedDamage + CanBeAttacked + ParryThreshold + IsUnconscious + MayHaveDurationDamage,
    {
        match self.target_body_part() {
            Some(part) => {
                let body_part = victim.body_mut().body_part_mut(part);
                if self.injury().is_some() {
                    body_part.add_injury(self.take_injury().unwrap());
                }
                if self.rupture_damage().is_some() && body_part.is_protected() {
                    body_part.protected_by_mut().unwrap().damage_rupture(self.rupture_damage().unwrap())
                }
            },
            None => {},
        }
        match self.kind() {
            CriticalHitKind::KnockedOut => victim.set_unconscious(),
            CriticalHitKind::SeveredGenitals |
            CriticalHitKind::VitalOrganDamage => {
                let reason = match self.kind() {
                    CriticalHitKind::SeveredGenitals => format!("{} severed his genitals", victim.name()),
                    CriticalHitKind::VitalOrganDamage => format!("{} damage a vital organ", victim.name()),
                    _ => panic!("Match should not be possible")
                };
                victim.add_duration_damage(reason, 1)
            },
            _ => {},
        }
        let damage = self.apply_damage_modifier(victim.roll_damage());
        // victim.take_damage(damage);
        DamageSummary::new(damage)
    }
}

pub trait CriticalHit {
    fn critical_hit<V: HasBody>(&self, victim: &V) -> CriticalHitResult;
}

impl<A: MayHaveWeapon> CriticalHit for A {
    fn critical_hit<V: HasBody>(&self, victim: &V) -> CriticalHitResult {
        match self.weapon() {
            None => panic!("Can't critical hit without weapon"),
            Some(weapon) => if weapon.is_sharp() {
                CriticalHitResult::roll_sharp(victim)
            } else {
                CriticalHitResult::roll_blunt(victim)
            }
        }
    }
}

impl MayTargetBodyPart for CriticalHitResult {
    fn target_body_part(&self) -> Option<&BodyPartKind> {
        self.body_part_kind.as_ref()
    }
}

impl MayHaveTestedRupture for CriticalHitResult {
    fn rupture_test_result(&self) -> Option<&RuptureTestResult> {
        self.rupture_test_result.as_ref()
    }
}

impl ApplyDamageModifier for CriticalHitResult {
    fn apply_damage_modifier(&self, base: u8) -> u8 {
        self.dmg_modifier.apply(base)
    }
}

impl MayHaveRuptureDamage for CriticalHitResult {
    fn rupture_damage(&self) -> Option<u8> {
        self.rupture_damage
    }
}

impl MayCauseInjury for CriticalHitResult {
    fn injury(&self) -> Option<&Injury> {
        self.injury.as_ref()
    }
}

impl TakeInjury for CriticalHitResult {
    fn take_injury(&mut self) -> Option<Injury> {
        self.injury.take()
    }
}

impl ExecuteAction for CriticalHitResult {
    fn execute<A, V>(&mut self, assailant: &mut A, victim: &mut V) -> DamageSummary
    where
        A: RollDamage + CanMissParries + CanMissAssaults + MayHaveWeapon + MayHaveMutableWeapon + TakeWeapon + HasName + HasBody + TakeDamage + TakeReducedDamage + CanBeAttacked + ParryThreshold,
        V: Assault + CriticalHit + HasName + MayHaveWeapon + IsUnconscious + HasMutableBody + TakeDamage + MayHaveDurationDamage,
    {
        match self.target_body_part() {
            Some(part) => {
                let body_part = victim.body_mut().body_part_mut(part);
                if self.injury().is_some() {
                    body_part.add_injury(self.take_injury().unwrap());
                }
                if self.rupture_damage().is_some() && body_part.is_protected() {
                    body_part.protected_by_mut().unwrap().damage_rupture(self.rupture_damage().unwrap())
                }
            },
            None => {},
        }
        match self.kind() {
            CriticalHitKind::KnockedOut => victim.set_unconscious(),
            CriticalHitKind::SeveredGenitals |
            CriticalHitKind::VitalOrganDamage => {
                let reason = match self.kind() {
                    CriticalHitKind::SeveredGenitals => format!("{} severed his genitals", assailant.name()),
                    CriticalHitKind::VitalOrganDamage => format!("{} damage a vital organ", assailant.name()),
                    _ => panic!("Match should not be possible")
                };
                victim.add_duration_damage(reason, 1)
            },
            _ => {},
        }
        let damage = self.apply_damage_modifier(assailant.roll_damage());
        // victim.take_damage(damage);
        DamageSummary::new(damage)
    }
}

impl ShowAction for CriticalHitResult {
    fn show<A, V>(&self, assailant: &A, victim: &V)
    where
        A: MayHaveWeapon + HasName,
        V: HasName + HasBody,
    {
        match self.kind() {
            CriticalHitKind::AccurateHeavyBlowAndArmorDamage => {
                println!(
                    "{} hits {} heavily, damaging his {}",
                    assailant.name(),
                    victim.name(),
                    self.display_protection_or_limb(victim),
                );
            }
            CriticalHitKind::BrokenArm | CriticalHitKind::BrokenHand | CriticalHitKind::BrokenLeg => {
                println!("{} broke {}'s {}", assailant.name(), victim.name(), self.target_body_part().unwrap());
            }
            CriticalHitKind::CrushedGenitals => {
                let genitals = self.target_body_part().unwrap();
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
                let eye = victim.body().body_part(self.target_body_part().unwrap());
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
            CriticalHitKind::PartOfTheArmorIsDestroyed => {
                match self.rupture_test_result() {
                    Some(result) => match result {
                        RuptureTestResult::Fail => println!(
                            "{} destroyed {}'s {}",
                            assailant.name(),
                            victim.name(),
                            self.display_protection_or_limb(victim),
                        ),
                        RuptureTestResult::Success => println!(
                            "{} hits {} savagely, damaging his {}",
                            assailant.name(),
                            victim.name(),
                            self.display_protection_or_limb(victim),
                        )
                    },
                    None => println!(
                        "{} hits {}'s unprotected {}.",
                        assailant.name(),
                        victim.name(),
                        self.target_body_part().unwrap(),
                    )
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
                let body_part = victim.body().body_part(self.target_body_part().unwrap());
                if body_part.is_severed() {
                    println!("{} slashed right where {}'s {} should be", assailant.name(), victim.name(), body_part.kind());
                } else {
                    println!("{} severed {}'s {}", assailant.name(), victim.name(), body_part.kind());
                }
            }
            CriticalHitKind::SmashedFoot => {
                let body_part = victim.body().body_part(self.target_body_part().unwrap());
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
