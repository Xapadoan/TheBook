use crate::dice::Dice;
use crate::equipment::weapon::OptionalMutableWeapon;

use super::CriticalHit;

pub trait DealCriticalHit: OptionalMutableWeapon {
    fn deal_critical_hit(&self) -> CriticalHit {
        match self.weapon() {
            Some(weapon) => {
                if weapon.is_sharp() {
                    sharp_critical()
                } else {
                    blunt_critical()
                }
            },
            None => panic!("No weapon")
        }
    }
}

pub fn sharp_critical() -> CriticalHit {
    match Dice::D20.roll() {
        1 | 2 => CriticalHit::DeepIncision,
        3 | 4 => CriticalHit::ReallyDeepIncision,
        5 | 6 => CriticalHit::ImpressiveWoundAndArmorDamage,
        7 | 8 => CriticalHit::PreciseHitAndArmorDamage,
        9 | 10 => CriticalHit::AccurateHeavyBlowAndArmorDamage,
        11 => CriticalHit::PartOfTheArmorIsDestroyed,
        12 => CriticalHit::GougedEye,
        13 => CriticalHit::SeveredHand,
        14 => CriticalHit::SeveredFoot,
        15 => CriticalHit::SeveredArm,
        16 => CriticalHit::SeveredLeg,
        17 => CriticalHit::WoundedGenitals,
        18 => CriticalHit::VitalOrganDamage,
        19 => CriticalHit::HeartInjury,
        20 => CriticalHit::SeriousHeadWound,
        other => panic!("D20 roll resulted in {other}"),
    }
}

pub fn blunt_critical() -> CriticalHit {
    match Dice::D20.roll() {
        1 | 2 => CriticalHit::ImpressiveBruise,
        3 | 4 => CriticalHit::ImpressiveBruiseAndLimbDislocation,
        5 | 6 => CriticalHit::RibFacture,
        7 | 8 => CriticalHit::PreciseHitAndArmorDamage,
        9 | 10 => CriticalHit::AccurateHeavyBlowAndArmorDamage,
        11 => CriticalHit::PartOfTheArmorIsDestroyed,
        12 => CriticalHit::KneeDislocation,
        13 => CriticalHit::BrokenHand,
        14 => CriticalHit::SmashedFoot,
        15 => CriticalHit::BrokenArm,
        16 => CriticalHit::BrokenLeg,
        17 => CriticalHit::CrushedGenitals,
        18 => CriticalHit::KnockedOut,
        19 => CriticalHit::OpenSkullFacture,
        20 => CriticalHit::VitalOrganCrushed,
        other => panic!("D20 roll resulted in {other}"),
    }
}
