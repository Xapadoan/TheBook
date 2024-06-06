use crate::dice::Dice;
use super::fight_action::{ApplyFightActionResult, ShowFightActionResult};
use super::{RollDamage, TakeDamage};
use crate::modifiers::Modifier;
use crate::warrior::Warrior;
use std::i8::MAX;

enum CriticalHitKind {
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

pub struct CriticalHitConsequence {
    kind: CriticalHitKind,
    dmg_modifier: Modifier,
}

impl CriticalHitConsequence {
    pub fn modify_damages(&self, base: u8) -> u8 {
        self.dmg_modifier.apply(base)
    }
}

impl ShowFightActionResult for CriticalHitConsequence {
    fn show_fight_action_result(&self, assailant: &Warrior, victim: &Warrior) {
        match self.kind {
            CriticalHitKind::AccurateHeavyBlowAndArmorDamage => {
                println!(
                    "{} hit {} heavily, damaging his armor",
                    assailant.name(),
                    victim.name()
                );
            }
            CriticalHitKind::BrokenArm => {
                println!("{} broke {}'s arm", assailant.name(), victim.name());
            }
            CriticalHitKind::BrokenHand => {
                println!("{} broke {}'s arm", assailant.name(), victim.name());
            }
            CriticalHitKind::BrokenLeg => {
                println!("{} broke {}'s leg", assailant.name(), victim.name());
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
                    "{} wounded {} deeply, damaging his armor",
                    assailant.name(),
                    victim.name()
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
                println!(
                    "{} destroyed a part of {}'s armor",
                    assailant.name(),
                    victim.name()
                );
            }
            CriticalHitKind::PreciseHitAndArmorDamage => {
                println!(
                    "{} hit {} precisely, damaging his armor",
                    assailant.name(),
                    victim.name()
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
            CriticalHitKind::SeveredArm => {
                println!("{} severed {}'s arm", assailant.name(), victim.name());
            }
            CriticalHitKind::SeveredFoot => {
                println!("{} severed {}'s foot", assailant.name(), victim.name());
            }
            CriticalHitKind::SeveredHand => {
                println!("{} severed {}'s hand", assailant.name(), victim.name());
            }
            CriticalHitKind::SeveredLeg => {
                println!("{} severed {}'s leg", assailant.name(), victim.name());
            }
            CriticalHitKind::SmashedFoot => {
                println!("{} smashed {}'s foot", assailant.name(), victim.name());
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

impl ApplyFightActionResult for CriticalHitConsequence {
    fn apply_fight_action_result(&self, assailant: &mut Warrior, victim: &mut Warrior) {
        let damage = self.modify_damages(assailant.roll_damage());
        victim.take_damage(damage);
    }
}

pub fn roll_sharp_critical() -> CriticalHitConsequence {
    match Dice::D20.roll() {
        1 | 2 => CriticalHitConsequence {
            kind: CriticalHitKind::DeepIncision,
            dmg_modifier: Modifier::new(1),
        },
        3 | 4 => CriticalHitConsequence {
            kind: CriticalHitKind::ReallyDeepIncision,
            dmg_modifier: Modifier::new(2),
        },
        5 | 6 => CriticalHitConsequence {
            kind: CriticalHitKind::ImpressiveWoundAndArmorDamage,
            dmg_modifier: Modifier::new(3),
        },
        7 | 8 => CriticalHitConsequence {
            kind: CriticalHitKind::PreciseHitAndArmorDamage,
            dmg_modifier: Modifier::new(4),
        },
        9 | 10 => CriticalHitConsequence {
            kind: CriticalHitKind::AccurateHeavyBlowAndArmorDamage,
            dmg_modifier: Modifier::new(5),
        },
        11 => CriticalHitConsequence {
            kind: CriticalHitKind::PartOfTheArmorIsDestroyed,
            dmg_modifier: Modifier::new(0),
        },
        12 => CriticalHitConsequence {
            kind: CriticalHitKind::GougedEye,
            dmg_modifier: Modifier::new(5),
        },
        13 => CriticalHitConsequence {
            kind: CriticalHitKind::SeveredHand,
            dmg_modifier: Modifier::new(6),
        },
        14 => CriticalHitConsequence {
            kind: CriticalHitKind::SeveredFoot,
            dmg_modifier: Modifier::new(6),
        },
        15 => CriticalHitConsequence {
            kind: CriticalHitKind::SeveredArm,
            dmg_modifier: Modifier::new(7),
        },
        16 => CriticalHitConsequence {
            kind: CriticalHitKind::SeveredLeg,
            dmg_modifier: Modifier::new(8),
        },
        17 => CriticalHitConsequence {
            kind: CriticalHitKind::GenitalsDamage,
            dmg_modifier: Modifier::new(5),
        },
        18 => CriticalHitConsequence {
            kind: CriticalHitKind::VitalOrganDamage,
            dmg_modifier: Modifier::new(9),
        },
        19 => CriticalHitConsequence {
            kind: CriticalHitKind::HeartInjury,
            dmg_modifier: Modifier::new(MAX),
        },
        20 => CriticalHitConsequence {
            kind: CriticalHitKind::SeriousHeadInjury,
            dmg_modifier: Modifier::new(MAX),
        },
        other => panic!("D20 roll resulted in {other}"),
    }
}

pub fn roll_blunt_critical() -> CriticalHitConsequence {
    match Dice::D20.roll() {
        1 | 2 => CriticalHitConsequence {
            kind: CriticalHitKind::ImpressiveBruise,
            dmg_modifier: Modifier::new(1),
        },
        3 | 4 => CriticalHitConsequence {
            kind: CriticalHitKind::ImpressiveBruiseAndLimbDislocation,
            dmg_modifier: Modifier::new(2),
        },
        5 | 6 => CriticalHitConsequence {
            kind: CriticalHitKind::RibFacture,
            dmg_modifier: Modifier::new(3),
        },
        7 | 8 => CriticalHitConsequence {
            kind: CriticalHitKind::PreciseHitAndArmorDamage,
            dmg_modifier: Modifier::new(4),
        },
        9 | 10 => CriticalHitConsequence {
            kind: CriticalHitKind::AccurateHeavyBlowAndArmorDamage,
            dmg_modifier: Modifier::new(5),
        },
        11 => CriticalHitConsequence {
            kind: CriticalHitKind::PartOfTheArmorIsDestroyed,
            dmg_modifier: Modifier::new(0),
        },
        12 => CriticalHitConsequence {
            kind: CriticalHitKind::KneeDislocation,
            dmg_modifier: Modifier::new(3),
        },
        13 => CriticalHitConsequence {
            kind: CriticalHitKind::BrokenHand,
            dmg_modifier: Modifier::new(3),
        },
        14 => CriticalHitConsequence {
            kind: CriticalHitKind::SmashedFoot,
            dmg_modifier: Modifier::new(3),
        },
        15 => CriticalHitConsequence {
            kind: CriticalHitKind::BrokenArm,
            dmg_modifier: Modifier::new(4),
        },
        16 => CriticalHitConsequence {
            kind: CriticalHitKind::BrokenLeg,
            dmg_modifier: Modifier::new(5),
        },
        17 => CriticalHitConsequence {
            kind: CriticalHitKind::CrushedGenitals,
            dmg_modifier: Modifier::new(5),
        },
        18 => CriticalHitConsequence {
            kind: CriticalHitKind::KnockedOut,
            dmg_modifier: Modifier::new(0),
        },
        19 => CriticalHitConsequence {
            kind: CriticalHitKind::OpenSkullFacture,
            dmg_modifier: Modifier::new(MAX),
        },
        20 => CriticalHitConsequence {
            kind: CriticalHitKind::VitalOrganCrushed,
            dmg_modifier: Modifier::new(MAX),
        },
        other => panic!("D20 roll resulted in {other}"),
    }
}
