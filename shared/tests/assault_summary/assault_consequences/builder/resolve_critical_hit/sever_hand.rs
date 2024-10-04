use shared::{
    assault::critical_hit::{CriticalHit, ResolveCriticalHit},
    equipment::{
        protection::{OptionalMutableProtection, ProtectionKind},
        rupture::RUPTURE_MAX,
    },
    warrior::body::{
        body_part::{BodyPartKind, BodySide, OptionalMutableBodyPart},
        HasMutableBody,
    },
};

use crate::common::{unbreakable_protection, unreliable_protection, TestAssailant};

#[test]
fn sever_hand_protection_rupture_success() {
    let mut victim = TestAssailant::new();
    let unbreakable_glove = unbreakable_protection(ProtectionKind::Gloves);
    let hand = victim
        .body_mut()
        .body_part_mut(&BodyPartKind::Hand(BodySide::Right))
        .as_mut()
        .unwrap();
    hand.replace_protection(unbreakable_glove);
    let unbreakable_glove = unbreakable_protection(ProtectionKind::Gloves);
    let hand = victim
        .body_mut()
        .body_part_mut(&BodyPartKind::Hand(BodySide::Left))
        .as_mut()
        .unwrap();
    hand.replace_protection(unbreakable_glove);

    let consequences = victim.resolve_critical_hit(0, &CriticalHit::SeveredHand);
    assert!(consequences.injury().is_none());
    assert!(consequences.armor_damages().is_some());
    let armor_damages = consequences.armor_damages().as_ref().unwrap();
    assert_eq!(armor_damages.damages(), 1);
}

#[test]
fn sever_hand_protection_rupture_fail() {
    let mut victim = TestAssailant::new();
    let unreliable_glove = unreliable_protection(ProtectionKind::Gloves);
    let hand = victim
        .body_mut()
        .body_part_mut(&BodyPartKind::Hand(BodySide::Right))
        .as_mut()
        .unwrap();
    hand.replace_protection(unreliable_glove);
    let unreliable_glove = unreliable_protection(ProtectionKind::Gloves);
    let hand = victim
        .body_mut()
        .body_part_mut(&BodyPartKind::Hand(BodySide::Left))
        .as_mut()
        .unwrap();
    hand.replace_protection(unreliable_glove);

    let consequences = victim.resolve_critical_hit(0, &CriticalHit::SeveredHand);
    assert!(consequences.injury().is_some());
    assert!(consequences.armor_damages().is_some());
    let armor_damages = consequences.armor_damages().as_ref().unwrap();
    assert_eq!(armor_damages.damages(), RUPTURE_MAX);
}
