use shared::{
    assault::critical_hit::{CriticalHit, ResolveCriticalHit},
    equipment::{
        protection::{OptionalMutableProtection, ProtectionKind},
        rupture::RUPTURE_MAX,
    }, warrior::body::{
        body_part::{BodyPartKind, BodySide, OptionalMutableBodyPart},
        HasMutableBody,
    }
};

use crate::common::{unbreakable_protection, unreliable_protection, TestAssailant};

#[test]
fn sever_foot_protection_rupture_success() {
    let mut victim = TestAssailant::new();
    let unbreakable_boot = unbreakable_protection(ProtectionKind::Boots);
    let foot = victim
        .body_mut()
        .body_part_mut(&BodyPartKind::Foot(BodySide::Right))
        .as_mut()
        .unwrap();
    foot.replace_protection(unbreakable_boot);
    let unbreakable_boot = unbreakable_protection(ProtectionKind::Boots);
    let foot = victim
        .body_mut()
        .body_part_mut(&BodyPartKind::Foot(BodySide::Left))
        .as_mut()
        .unwrap();
    foot.replace_protection(unbreakable_boot);

    let consequences = victim.resolve_critical_hit(0, &CriticalHit::SeveredFoot);
    assert!(consequences.injury().is_none());
    assert!(consequences.armor_damages().is_some());
    let armor_damages = consequences.armor_damages().as_ref().unwrap();
    assert_eq!(armor_damages.damages(), 1);
}

#[test]
fn sever_foot_protection_rupture_fail() {
    let mut victim = TestAssailant::new();
    let unreliable_boot = unreliable_protection(ProtectionKind::Boots);
    let foot = victim
        .body_mut()
        .body_part_mut(&BodyPartKind::Foot(BodySide::Right))
        .as_mut()
        .unwrap();
    foot.replace_protection(unreliable_boot);
    let unreliable_boot = unreliable_protection(ProtectionKind::Boots);
    let foot = victim
        .body_mut()
        .body_part_mut(&BodyPartKind::Foot(BodySide::Left))
        .as_mut()
        .unwrap();
    foot.replace_protection(unreliable_boot);

    let consequences = victim.resolve_critical_hit(0, &CriticalHit::SeveredFoot);
    assert!(consequences.injury().is_some());
    assert!(consequences.armor_damages().is_some());
    let armor_damages = consequences.armor_damages().as_ref().unwrap();
    assert_eq!(armor_damages.damages(), RUPTURE_MAX);
}
