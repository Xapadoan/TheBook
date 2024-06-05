use crate::dice::Dice;
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

pub struct CriticalConsequence {
    kind: CriticalHitKind,
    dmg_modifier: Modifier,
}

impl CriticalConsequence {
    pub fn modify_damages(&self, base: u8) -> u8 {
        self.dmg_modifier.apply(base)
    }

    pub fn show(&self, assailant: &Warrior, victim: &Warrior) {
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

pub fn roll_sharp_critical() -> CriticalConsequence {
    match Dice::D20.roll() {
        1 | 2 => CriticalConsequence {
            kind: CriticalHitKind::DeepIncision,
            dmg_modifier: Modifier::new(1),
        },
        3 | 4 => CriticalConsequence {
            kind: CriticalHitKind::ReallyDeepIncision,
            dmg_modifier: Modifier::new(2),
        },
        5 | 6 => CriticalConsequence {
            kind: CriticalHitKind::ImpressiveWoundAndArmorDamage,
            dmg_modifier: Modifier::new(3),
        },
        7 | 8 => CriticalConsequence {
            kind: CriticalHitKind::PreciseHitAndArmorDamage,
            dmg_modifier: Modifier::new(4),
        },
        9 | 10 => CriticalConsequence {
            kind: CriticalHitKind::AccurateHeavyBlowAndArmorDamage,
            dmg_modifier: Modifier::new(5),
        },
        11 => CriticalConsequence {
            kind: CriticalHitKind::PartOfTheArmorIsDestroyed,
            dmg_modifier: Modifier::new(0),
        },
        12 => CriticalConsequence {
            kind: CriticalHitKind::GougedEye,
            dmg_modifier: Modifier::new(5),
        },
        13 => CriticalConsequence {
            kind: CriticalHitKind::SeveredHand,
            dmg_modifier: Modifier::new(6),
        },
        14 => CriticalConsequence {
            kind: CriticalHitKind::SeveredFoot,
            dmg_modifier: Modifier::new(6),
        },
        15 => CriticalConsequence {
            kind: CriticalHitKind::SeveredArm,
            dmg_modifier: Modifier::new(7),
        },
        16 => CriticalConsequence {
            kind: CriticalHitKind::SeveredLeg,
            dmg_modifier: Modifier::new(8),
        },
        17 => CriticalConsequence {
            kind: CriticalHitKind::GenitalsDamage,
            dmg_modifier: Modifier::new(5),
        },
        18 => CriticalConsequence {
            kind: CriticalHitKind::VitalOrganDamage,
            dmg_modifier: Modifier::new(9),
        },
        19 => CriticalConsequence {
            kind: CriticalHitKind::HeartInjury,
            dmg_modifier: Modifier::new(MAX),
        },
        20 => CriticalConsequence {
            kind: CriticalHitKind::SeriousHeadInjury,
            dmg_modifier: Modifier::new(MAX),
        },
        other => panic!("D20 roll resulted in {other}"),
    }
}

pub fn roll_blunt_critical() -> CriticalConsequence {
    match Dice::D20.roll() {
        1 | 2 => CriticalConsequence {
            kind: CriticalHitKind::ImpressiveBruise,
            dmg_modifier: Modifier::new(1),
        },
        3 | 4 => CriticalConsequence {
            kind: CriticalHitKind::ImpressiveBruiseAndLimbDislocation,
            dmg_modifier: Modifier::new(2),
        },
        5 | 6 => CriticalConsequence {
            kind: CriticalHitKind::RibFacture,
            dmg_modifier: Modifier::new(3),
        },
        7 | 8 => CriticalConsequence {
            kind: CriticalHitKind::PreciseHitAndArmorDamage,
            dmg_modifier: Modifier::new(4),
        },
        9 | 10 => CriticalConsequence {
            kind: CriticalHitKind::AccurateHeavyBlowAndArmorDamage,
            dmg_modifier: Modifier::new(5),
        },
        11 => CriticalConsequence {
            kind: CriticalHitKind::PartOfTheArmorIsDestroyed,
            dmg_modifier: Modifier::new(0),
        },
        12 => CriticalConsequence {
            kind: CriticalHitKind::KneeDislocation,
            dmg_modifier: Modifier::new(3),
        },
        13 => CriticalConsequence {
            kind: CriticalHitKind::BrokenHand,
            dmg_modifier: Modifier::new(3),
        },
        14 => CriticalConsequence {
            kind: CriticalHitKind::SmashedFoot,
            dmg_modifier: Modifier::new(3),
        },
        15 => CriticalConsequence {
            kind: CriticalHitKind::BrokenArm,
            dmg_modifier: Modifier::new(4),
        },
        16 => CriticalConsequence {
            kind: CriticalHitKind::BrokenLeg,
            dmg_modifier: Modifier::new(5),
        },
        17 => CriticalConsequence {
            kind: CriticalHitKind::CrushedGenitals,
            dmg_modifier: Modifier::new(5),
        },
        18 => CriticalConsequence {
            kind: CriticalHitKind::KnockedOut,
            dmg_modifier: Modifier::new(0),
        },
        19 => CriticalConsequence {
            kind: CriticalHitKind::OpenSkullFacture,
            dmg_modifier: Modifier::new(MAX),
        },
        20 => CriticalConsequence {
            kind: CriticalHitKind::VitalOrganCrushed,
            dmg_modifier: Modifier::new(MAX),
        },
        other => panic!("D20 roll resulted in {other}"),
    }
}
