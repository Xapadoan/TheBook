use shared::assault::assault_consequence::IndividualConsequences;
use shared::assault::critical_hit::CriticalHit;
use shared::equipment::protection::OptionalMutableProtection;
use shared::warrior::body::body_part::{BodyPartKind, BodySide, OptionalBodyPart};
use shared::warrior::body::injury::Injury;

use crate::show::show_resolution::show_rupture;

use super::show_resolution::show_lose_eye;
use super::{ShowSelf, ReplayActor};

pub fn show_self_critical_hit(
    critical_hit: &CriticalHit,
    idiot: &dyn ReplayActor,
    consequences: &IndividualConsequences,
) -> String {
    match critical_hit {
        CriticalHit::ImpressiveWoundAndArmorDamage |
        CriticalHit::PreciseHitAndArmorDamage |
        CriticalHit::AccurateHeavyBlowAndArmorDamage |
        CriticalHit::PartOfTheArmorIsDestroyed  => show_armor_damage_resolution(idiot, consequences),
        CriticalHit::DeepIncision |
        CriticalHit::ReallyDeepIncision => format!(
            "{} cuts himself deeply",
            idiot.show_self(),
        ),
        CriticalHit::GougedEye => format!(
            "{} hits his eye. {}",
            idiot.show_self(),
            show_lose_eye(consequences.injury()),
        ),
        CriticalHit::SeveredArm |
        CriticalHit::SeveredLeg |
        CriticalHit::SeveredHand |
        CriticalHit::SeveredFoot => show_sever_limb(idiot, consequences),
        CriticalHit::WoundedGenitals => show_wound_genitals(idiot, consequences),
        CriticalHit::VitalOrganDamage => format!(
            "{} pierces his one lung. He starts to bleed a lot",
            idiot.show_self(),
        ),
        CriticalHit::HeartInjury => format!(
            "{}'s {} landed in his own heart",
            idiot.show_self(),
            idiot.weapon().as_ref().unwrap().show_self(),
        ),
        CriticalHit::SeriousHeadWound => format!(
            "{}'s {} went right through his own {}",
            idiot.show_self(),
            idiot.weapon().as_ref().unwrap().show_self(),
            BodyPartKind::Head.show_self(),
        ),
        CriticalHit::ImpressiveBruise |
        CriticalHit::ImpressiveBruiseAndLimbDislocation => format!(
            "{} hits himself heavily",
            idiot.show_self(),
        ),
        CriticalHit::RibFacture => format!(
            "{} hits one of his ribs, fracturing it",
            idiot.show_self(),
        ),
        CriticalHit::BrokenArm |
        CriticalHit::BrokenHand |
        CriticalHit::BrokenLeg |
        CriticalHit::SmashedFoot |
        CriticalHit::KneeDislocation => show_break_limb(
            critical_hit,
            idiot,
            consequences,
        ),
        CriticalHit::CrushedGenitals => format!(
            "{} crushes his own {}",
            idiot.show_self(),
            BodyPartKind::Genitals.show_self(),
        ),
        CriticalHit::KnockedOut => format!(
            "{} knocks himself out",
            idiot.show_self(),
        ),
        CriticalHit::OpenSkullFacture => format!(
            "{}'s {} ends up straight into his {} and cracks it open",
            idiot.show_self(),
            idiot.weapon().as_ref().unwrap().show_self(),
            BodyPartKind::Head.show_self(),
        ),
        CriticalHit::VitalOrganCrushed => format!(
            "{} crushes his own liver flat",
            idiot.show_self()
        )
    }
}

fn show_break_limb(
    critical_hit: &CriticalHit,
    idiot: &dyn ReplayActor,
    consequences: &IndividualConsequences,
) -> String {
    if consequences.damages() == 0 {
        let missing_limb = match critical_hit {
            CriticalHit::BrokenArm => "missing arm",
            CriticalHit::BrokenHand => "missing hand",
            CriticalHit::BrokenLeg => "missing leg",
            CriticalHit::SeveredFoot => "missing foot",
            CriticalHit::KneeDislocation => "missing knee",
            _ => panic!("Impossible match"),
        };
        return format!(
            "{} hits the air where his {} should have been",
            idiot.show_self(),
            missing_limb,
        )
    }
    let limb = match critical_hit {
        CriticalHit::BrokenArm => "arm",
        CriticalHit::BrokenHand => "hand",
        CriticalHit::BrokenLeg => "leg",
        CriticalHit::SeveredFoot => "foot",
        CriticalHit::KneeDislocation => "knee",
        _ => panic!("Impossible match"),
    };
    if let Some(_) = consequences.injury() {
        format!(
            "{}'s {} pounds his {}, breaking it",
            idiot.show_self(),
            idiot.weapon().as_ref().unwrap().show_self(),
            limb,
        )
    } else {
        format!(
            "{} hits himself heavily on his broken {}",
            idiot.show_self(),
            limb
        )
    }
}

fn show_wound_genitals(
    idiot: &dyn ReplayActor,
    consequences: &IndividualConsequences,
) -> String {
    if let Some(_) = consequences.duration_damages() {
        format!(
            "{} slices through his own {}",
            idiot.show_self(),
            BodyPartKind::Genitals.show_self(),
        )
    } else {
        format!(
            "{} hits right where his {} should have been",
            idiot.show_self(),
            BodyPartKind::Genitals.show_self(),
        )
    }
}

fn show_sever_limb(
    idiot: &dyn ReplayActor,
    consequences: &IndividualConsequences,
) -> String {
    if let Some(injury) = consequences.injury() {
        let severed_limb = match injury {
            Injury::LeftArmSevered => {
                let arm = BodyPartKind::Arm(BodySide::Left);
                arm.show_self()
            },
            Injury::RightArmSevered => BodyPartKind::Arm(BodySide::Right).show_self(),
            Injury::LeftHandSevered => BodyPartKind::Hand(BodySide::Left).show_self(),
            Injury::RightHandSevered => BodyPartKind::Hand(BodySide::Right).show_self(),
            Injury::OneLegSevered(side) => BodyPartKind::Leg(side.clone()).show_self(),
            Injury::BothLegsSevered => "remaining leg".to_string(),
            Injury::FootSevered(side) => BodyPartKind::Foot(side.clone()).show_self(),
            _ => panic!("Impossible match"),
        };
        format!(
            "{}'s {} swigs toward his {} and cuts it in a gruesome way",
            idiot.show_self(),
            idiot.weapon().as_ref().unwrap().show_self(),
            severed_limb
        )
    } else if let Some(_) = consequences.armor_damages() {
        show_armor_damage_resolution(idiot, consequences)
    } else {
        format!(
            "{} cuts himself deeply",
            idiot.show_self(),
        )
    }
}

fn show_armor_damage_resolution(
    idiot: &dyn ReplayActor,
    consequences: &IndividualConsequences,
) -> String {
    if let Some(armor_damages) = consequences.armor_damages() {
        let body_part_kind = armor_damages.body_part_kind();
        let rupture_damages = armor_damages.damages();
        let body_part = idiot.body().body_part(body_part_kind).as_ref().unwrap();
        let protection = body_part.protection().as_ref().unwrap();

        format!(
            "{}'s {} hits heavily on his {}, {}.",
            idiot.show_self(),
            idiot.weapon().as_ref().unwrap().show_self(),
            protection.show_self(),
            show_rupture(protection, rupture_damages),
        )
    } else {
        format!("{} hits himself violently", idiot.show_self())
    }
}
