use crate::dice::Dice;
use super::fight_action::{ApplyFightActionResult, ShowFightActionResult};
use super::{IsUnconscious, RollDamage, TakeDamage};
use crate::warrior::Warrior;
use std::u8::MAX;

pub enum CriticalHitResult {
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

impl ShowFightActionResult for CriticalHitResult {
    fn show_fight_action_result(&self, assailant: &Warrior, victim: &Warrior) {
        match self {
            CriticalHitResult::AccurateHeavyBlowAndArmorDamage => {
                println!(
                    "{} hit {} heavily, damaging his armor",
                    assailant.name(),
                    victim.name()
                );
            }
            CriticalHitResult::BrokenArm => {
                println!("{} broke {}'s arm", assailant.name(), victim.name());
            }
            CriticalHitResult::BrokenHand => {
                println!("{} broke {}'s arm", assailant.name(), victim.name());
            }
            CriticalHitResult::BrokenLeg => {
                println!("{} broke {}'s leg", assailant.name(), victim.name());
            }
            CriticalHitResult::CrushedGenitals => {
                println!("{} crushed {}'s genitals", assailant.name(), victim.name());
            }
            CriticalHitResult::DeepIncision => {
                println!("{} cut {} deeply", assailant.name(), victim.name());
            }
            CriticalHitResult::GenitalsDamage => {
                println!("{} hit {}'s genitals", assailant.name(), victim.name());
            }
            CriticalHitResult::GougedEye => {
                println!("{} gouged {}'s eye", assailant.name(), victim.name());
            }
            CriticalHitResult::HeartInjury => {
                println!("{} pierced {}'s heart", assailant.name(), victim.name());
            }
            CriticalHitResult::ImpressiveBruise => {
                println!("{} bruised {} heavily", assailant.name(), victim.name());
            }
            CriticalHitResult::ImpressiveBruiseAndLimbDislocation => {
                println!(
                    "{} bruised {} heavily, dislocating a limb",
                    assailant.name(),
                    victim.name()
                );
            }
            CriticalHitResult::ImpressiveWoundAndArmorDamage => {
                println!(
                    "{} wounded {} deeply, damaging his armor",
                    assailant.name(),
                    victim.name()
                );
            }
            CriticalHitResult::KneeDislocation => {
                println!("{} dislocated {}'s knee", assailant.name(), victim.name());
            }
            CriticalHitResult::KnockedOut => {
                println!("{} knocked {} out", assailant.name(), victim.name());
            }
            CriticalHitResult::OpenSkullFacture => {
                println!("{} opened {}'s skull wide", assailant.name(), victim.name());
            }
            CriticalHitResult::PartOfTheArmorIsDestroyed => {
                println!(
                    "{} destroyed a part of {}'s armor",
                    assailant.name(),
                    victim.name()
                );
            }
            CriticalHitResult::PreciseHitAndArmorDamage => {
                println!(
                    "{} hit {} precisely, damaging his armor",
                    assailant.name(),
                    victim.name()
                );
            }
            CriticalHitResult::ReallyDeepIncision => {
                println!("{} cut {} really deep", assailant.name(), victim.name());
            }
            CriticalHitResult::RibFacture => {
                println!("{} fractured {}'s rib", assailant.name(), victim.name());
            }
            CriticalHitResult::SeriousHeadInjury => {
                println!("{} cut through {}'s head", assailant.name(), victim.name());
            }
            CriticalHitResult::SeveredArm => {
                println!("{} severed {}'s arm", assailant.name(), victim.name());
            }
            CriticalHitResult::SeveredFoot => {
                println!("{} severed {}'s foot", assailant.name(), victim.name());
            }
            CriticalHitResult::SeveredHand => {
                println!("{} severed {}'s hand", assailant.name(), victim.name());
            }
            CriticalHitResult::SeveredLeg => {
                println!("{} severed {}'s leg", assailant.name(), victim.name());
            }
            CriticalHitResult::SmashedFoot => {
                println!("{} smashed {}'s foot", assailant.name(), victim.name());
            }
            CriticalHitResult::VitalOrganCrushed => {
                println!(
                    "{} crushed one of {}'s vital organs",
                    assailant.name(),
                    victim.name()
                );
            }
            CriticalHitResult::VitalOrganDamage => {
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
        match self {
            CriticalHitResult::DeepIncision => damage += 1,
            CriticalHitResult::ReallyDeepIncision => damage += 2,
            CriticalHitResult::ImpressiveWoundAndArmorDamage => {
                println!("[WARN] damage to armor is not implemented");
                damage += 3;
            },
            CriticalHitResult::PreciseHitAndArmorDamage => {
                println!("[WARN] damage to armor is not implemented");
                damage += 4;
            },
            CriticalHitResult::AccurateHeavyBlowAndArmorDamage => {
                println!("[WARN] damage to armor is not implemented");
                damage += 5;
            },
            CriticalHitResult::PartOfTheArmorIsDestroyed => println!("[WARN] damage to armor is not implemented"),
            CriticalHitResult::GougedEye => {
                println!("[WARN] deep wounds not implemented");
                damage += 5;
            },
            CriticalHitResult::SeveredHand => {
                println!("[WARN] deep wounds not implemented");
                damage += 6;
            },
            CriticalHitResult::SeveredFoot => {
                println!("[WARN] deep wounds not implemented");
                damage += 6;
            },
            CriticalHitResult::SeveredArm => {
                println!("[WARN] deep wounds not implemented");
                damage += 7;
            },
            CriticalHitResult::SeveredLeg => {
                println!("[WARN] deep wounds not implemented");
                damage += 8;
            },
            CriticalHitResult::GenitalsDamage => {
                println!("[WARN] duration damage is not implemented");
                damage += 5;
            },
            CriticalHitResult::VitalOrganDamage => {
                println!("[WARN] duration damage is not implemented");
                damage += 9;
            },
            CriticalHitResult::HeartInjury => damage = MAX,
            CriticalHitResult::SeriousHeadInjury => damage = MAX,
            CriticalHitResult::ImpressiveBruise => damage += 1,
            CriticalHitResult::ImpressiveBruiseAndLimbDislocation => damage += 2,
            CriticalHitResult::RibFacture => damage += 3,
            CriticalHitResult::KneeDislocation => {
                println!("[WARN] deep wounds not implemented");
                damage += 3;
            },
            CriticalHitResult::BrokenHand => {
                println!("[WARN] deep wounds not implemented");
                damage += 3;
            },
            CriticalHitResult::SmashedFoot => {
                println!("[WARN] deep wounds not implemented");
                damage += 3;
            },
            CriticalHitResult::BrokenArm => {
                println!("[WARN] deep wounds not implemented");
                damage += 4;
            },
            CriticalHitResult::BrokenLeg => {
                println!("[WARN] deep wounds not implemented");
                damage += 5;
            },
            CriticalHitResult::CrushedGenitals => {
                println!("[WARN] deep wounds not implemented");
                damage += 5;
            },
            CriticalHitResult::KnockedOut => victim.set_unconscious(),
            CriticalHitResult::OpenSkullFacture => damage = MAX,
            CriticalHitResult::VitalOrganCrushed => damage = MAX,
        }
        victim.take_damage(damage);
    }
}

pub fn roll_sharp_critical() -> CriticalHitResult {
    match Dice::D20.roll() {
        1 | 2 => CriticalHitResult::DeepIncision,
        3 | 4 => CriticalHitResult::ReallyDeepIncision,
        5 | 6 => CriticalHitResult::ImpressiveWoundAndArmorDamage,
        7 | 8 => CriticalHitResult::PreciseHitAndArmorDamage,
        9 | 10 => CriticalHitResult::AccurateHeavyBlowAndArmorDamage,
        11 => CriticalHitResult::PartOfTheArmorIsDestroyed,
        12 => CriticalHitResult::GougedEye,
        13 => CriticalHitResult::SeveredHand,
        14 => CriticalHitResult::SeveredFoot,
        15 => CriticalHitResult::SeveredArm,
        16 => CriticalHitResult::SeveredLeg,
        17 => CriticalHitResult::GenitalsDamage,
        18 => CriticalHitResult::VitalOrganDamage,
        19 => CriticalHitResult::HeartInjury,
        20 => CriticalHitResult::SeriousHeadInjury,
        other => panic!("D20 roll resulted in {other}"),
    }
}

pub fn roll_blunt_critical() -> CriticalHitResult {
    match Dice::D20.roll() {
        1 | 2 => CriticalHitResult::ImpressiveBruise,
        3 | 4 => CriticalHitResult::ImpressiveBruiseAndLimbDislocation,
        5 | 6 => CriticalHitResult::RibFacture,
        7 | 8 => CriticalHitResult::PreciseHitAndArmorDamage,
        9 | 10 => CriticalHitResult::AccurateHeavyBlowAndArmorDamage,
        11 => CriticalHitResult::PartOfTheArmorIsDestroyed,
        12 => CriticalHitResult::KneeDislocation,
        13 => CriticalHitResult::BrokenHand,
        14 => CriticalHitResult::SmashedFoot,
        15 => CriticalHitResult::BrokenArm,
        16 => CriticalHitResult::BrokenLeg,
        17 => CriticalHitResult::CrushedGenitals,
        18 => CriticalHitResult::KnockedOut,
        19 => CriticalHitResult::OpenSkullFacture,
        20 => CriticalHitResult::VitalOrganCrushed,
        other => panic!("D20 roll resulted in {other}"),
    }
}
