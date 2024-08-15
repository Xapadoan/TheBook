// Should test the to_consequences function here

use std::u8;

use serde::{Deserialize, Serialize};

use super::super::assailant::Assailant;
use super::super::assault_consequence::{AssaultConsequences, AssaultConsequencesBuilder, IndividualConsequences};

#[derive(Debug, Serialize, Deserialize)]
pub enum CriticalHit {
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
    WoundedGenitals,
    VitalOrganDamage,
    HeartInjury,
    SeriousHeadWound,
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

impl AssaultConsequencesBuilder for CriticalHit {
    // server only
    fn to_consequences(&self, assailant: & dyn Assailant, victim: & dyn Assailant) -> AssaultConsequences {
        let for_assailant = IndividualConsequences::no_consequences();
        let damages = assailant.deal_damages();
        let for_victim = victim.resolve_critical_hit(damages, self);
        AssaultConsequences::new(for_assailant, for_victim)
    }
}
