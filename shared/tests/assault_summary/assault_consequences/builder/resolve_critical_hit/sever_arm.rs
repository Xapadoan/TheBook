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
fn sever_arm_protection_rupture_success() {
    let mut victim = TestAssailant::new();
    let unbreakable_armlet = unbreakable_protection(ProtectionKind::Armlets);
    let arm = victim
        .body_mut()
        .body_part_mut(&BodyPartKind::Arm(BodySide::Right))
        .as_mut()
        .unwrap();
    arm.replace_protection(unbreakable_armlet);
    let unbreakable_armlet = unbreakable_protection(ProtectionKind::Armlets);
    let arm = victim
        .body_mut()
        .body_part_mut(&BodyPartKind::Arm(BodySide::Left))
        .as_mut()
        .unwrap();
    arm.replace_protection(unbreakable_armlet);

    let consequences = victim.resolve_critical_hit(0, &CriticalHit::SeveredArm);
    assert!(consequences.injury().is_none());
    assert!(consequences.armor_damages().is_some());
    let armor_damages = consequences.armor_damages().as_ref().unwrap();
    assert_eq!(armor_damages.damages(), 1);
}

#[test]
fn sever_arm_protection_rupture_fail() {
    let mut victim = TestAssailant::new();
    let unreliable_armlet = unreliable_protection(ProtectionKind::Armlets);
    let arm = victim
        .body_mut()
        .body_part_mut(&BodyPartKind::Arm(BodySide::Right))
        .as_mut()
        .unwrap();
    arm.replace_protection(unreliable_armlet);
    let unreliable_armlet = unreliable_protection(ProtectionKind::Armlets);
    let arm = victim
        .body_mut()
        .body_part_mut(&BodyPartKind::Arm(BodySide::Left))
        .as_mut()
        .unwrap();
    arm.replace_protection(unreliable_armlet);

    let consequences = victim.resolve_critical_hit(0, &CriticalHit::SeveredArm);
    assert!(consequences.injury().is_some());
    assert!(consequences.armor_damages().is_some());
    let armor_damages = consequences.armor_damages().as_ref().unwrap();
    assert_eq!(armor_damages.damages(), RUPTURE_MAX);
}
