use shared::assault::assault_consequence::AssaultConsequences;
use shared::assault::critical_hit::CriticalHit;
use shared::equipment::protection::OptionalMutableProtection;
use shared::warrior::body::body_part::{BodyPartKind, OptionalBodyPart, BodySide};
use shared::warrior::body::injury::Injury;

use crate::client::show::show_resolution::{show_lose_eye, show_rupture};
use crate::client::show::{ReplayActor, ShowSelf};

use super::ShowReplay;

impl ShowReplay for CriticalHit {
    fn show_replay(
        &self,
        assailant: &dyn ReplayActor,
        victim: &dyn ReplayActor,
        consequences: &AssaultConsequences,
    ) -> String {
        match self {
            CriticalHit::ImpressiveWoundAndArmorDamage |
            CriticalHit::PreciseHitAndArmorDamage |
            CriticalHit::AccurateHeavyBlowAndArmorDamage |
            CriticalHit::PartOfTheArmorIsDestroyed  => show_armor_damage_resolution(assailant, victim, consequences),
            CriticalHit::DeepIncision |
            CriticalHit::ReallyDeepIncision => format!(
                "{} cuts {} deeply",
                assailant.show_self(),
                victim.show_self(),
            ),
            CriticalHit::GougedEye => format!(
                "{} hits {}'s eye. {}",
                assailant.show_self(),
                victim.show_self(),
                show_lose_eye(consequences.for_victim().injury()),
            ),
            CriticalHit::SeveredArm |
            CriticalHit::SeveredLeg |
            CriticalHit::SeveredHand |
            CriticalHit::SeveredFoot => show_sever_limb(assailant, victim, consequences),
            CriticalHit::WoundedGenitals => show_wound_genitals(assailant, victim, consequences),
            CriticalHit::VitalOrganDamage => format!(
                "{} pierces one of {}'s lung. He starts to bleed a lot",
                assailant.show_self(),
                victim.show_self(),
            ),
            CriticalHit::HeartInjury => format!(
                "{}'s {} landed in {}'s heart",
                assailant.show_self(),
                assailant.weapon().as_ref().unwrap().show_self(),
                victim.show_self(),
            ),
            CriticalHit::SeriousHeadWound => format!(
                "{} slices through {}'s {}",
                assailant.show_self(),
                victim.show_self(),
                BodyPartKind::Head.show_self(),
            ),
            CriticalHit::ImpressiveBruise |
            CriticalHit::ImpressiveBruiseAndLimbDislocation => format!(
                "{} strikes {} heavily",
                assailant.show_self(),
                victim.show_self(),
            ),
            CriticalHit::RibFacture => format!(
                "{} hits one of {}'s ribs, fracturing it",
                assailant.show_self(),
                victim.show_self(),
            ),
            CriticalHit::BrokenArm |
            CriticalHit::BrokenHand |
            CriticalHit::BrokenLeg |
            CriticalHit::SmashedFoot |
            CriticalHit::KneeDislocation => show_break_limb(
                self,
                assailant,
                victim,
                consequences,
            ),
            CriticalHit::CrushedGenitals => format!(
                "{} crushes {}'s {}",
                assailant.show_self(),
                victim.show_self(),
                BodyPartKind::Genitals.show_self(),
            ),
            CriticalHit::KnockedOut => format!(
                "{} knocks {} out",
                assailant.show_self(),
                victim.show_self(),
            ),
            CriticalHit::OpenSkullFacture => format!(
                "{} swings his {} straight into {}'s {} and cracks it open",
                assailant.show_self(),
                assailant.weapon().as_ref().unwrap().show_self(),
                victim.show_self(),
                BodyPartKind::Head.show_self(),
            ),
            CriticalHit::VitalOrganCrushed => format!(
                "{} crushes {}'s liver flat",
                assailant.show_self(),
                victim.show_self()
            )
        }
    }
}

fn show_break_limb(
    critical_hit: &CriticalHit,
    assailant: &dyn ReplayActor,
    victim: &dyn ReplayActor,
    consequences: &AssaultConsequences,
) -> String {
    if consequences.for_victim().damages() == 0 {
        let missing_limb = match critical_hit {
            CriticalHit::BrokenArm => "missing arm",
            CriticalHit::BrokenHand => "missing hand",
            CriticalHit::BrokenLeg => "missing leg",
            CriticalHit::SmashedFoot => "missing foot",
            CriticalHit::KneeDislocation => "missing knee",
            _ => panic!("Impossible match"),
        };
        return format!(
            "{} hits the air where {}'s {} should have been",
            assailant.show_self(),
            victim.show_self(),
            missing_limb,
        )
    }
    let limb = match critical_hit {
        CriticalHit::BrokenArm => "arm",
        CriticalHit::BrokenHand => "hand",
        CriticalHit::BrokenLeg => "leg",
        CriticalHit::SmashedFoot => "foot",
        CriticalHit::KneeDislocation => "knee",
        _ => panic!("Impossible match"),
    };
    if let Some(_) = consequences.for_victim().injury() {
        format!(
            "{}'s {} pounds {}'s {}, breaking it",
            assailant.show_self(),
            assailant.weapon().as_ref().unwrap().show_self(),
            victim.show_self(),
            limb,
        )
    } else {
        format!(
            "{} strikes heavily on {}'s broken {}",
            assailant.show_self(),
            victim.show_self(),
            limb
        )
    }
}

fn show_wound_genitals(
    assailant: &dyn ReplayActor,
    victim: &dyn ReplayActor,
    consequences: &AssaultConsequences,
) -> String {
    if let Some(_) = consequences.for_victim().duration_damages() {
        format!(
            "{} slices through {}'s {}",
            assailant.show_self(),
            victim.show_self(),
            BodyPartKind::Genitals.show_self(),
        )
    } else {
        format!(
            "{} hits right where {}'s {} should have been",
            assailant.show_self(),
            victim.show_self(),
            BodyPartKind::Genitals.show_self(),
        )
    }
}

fn show_sever_limb(
    assailant: &dyn ReplayActor,
    victim: &dyn ReplayActor,
    consequences: &AssaultConsequences,
) -> String {
    if let Some(injury) = consequences.for_victim().injury() {
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
            "{} swigs his {} toward {}'s {} and cuts it in a gruesome way",
            assailant.show_self(),
            assailant.weapon().as_ref().unwrap().show_self(),
            victim.show_self(),
            severed_limb
        )
    } else if let Some(_) = consequences.for_victim().armor_damages() {
        show_armor_damage_resolution(assailant, victim, consequences)
    } else {
        format!(
            "{} cuts {} deeply",
            assailant.show_self(),
            victim.show_self()
        )
    }
}

fn show_armor_damage_resolution(
    assailant: &dyn ReplayActor,
    victim: &dyn ReplayActor,
    consequences: &AssaultConsequences,
) -> String {
    if let Some(armor_damages) = consequences.for_victim().armor_damages() {
        let body_part_kind = armor_damages.body_part_kind();
        let rupture_damages = armor_damages.damages();
        let body_part = victim.body().body_part(body_part_kind).as_ref().unwrap();
        let protection = body_part.protection().as_ref().unwrap();

        format!(
            "{}'s {} hits heavily on {}'s {}, {}.",
            assailant.show_self(),
            assailant.weapon().as_ref().unwrap().show_self(),
            victim.show_self(),
            protection.show_self(),
            show_rupture(protection, rupture_damages),
        )
    } else {
        format!("{} hits {} violently", assailant.show_self(), victim.show_self())
    }
}
