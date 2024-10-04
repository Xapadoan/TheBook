use std::u8;

use shared::{assault::assault_consequence::{ArmorDamages, AssaultConsequences, IndividualConsequences}, equipment::protection::{OptionalMutableProtection, Protection, ProtectionKind}, warrior::body::{body_part::{BodyPartKind, BodySide, OptionalBodyPart, OptionalMutableBodyPart}, HasBody, HasMutableBody}};

use crate::common::TestAssailant;

#[test]
fn victim_drops_protection_when_broken() {
    let mut assailant = TestAssailant::new();
    let mut victim = TestAssailant::new();
    let protection = Protection::new(ProtectionKind::Gloves);
    let body_part_kind = BodyPartKind::Hand(BodySide::Left);
    victim
        .body_mut()
        .body_part_mut(&body_part_kind)
        .as_mut()
        .unwrap()
        .replace_protection(protection);
    let consequences = AssaultConsequences::new(
        IndividualConsequences::no_consequences(),
        IndividualConsequences::damage_armor(
            0,
            ArmorDamages::new(u8::MAX, body_part_kind.clone())
        ),
    );

    consequences.apply(&mut assailant, &mut victim);

    assert!(
        victim.body()
            .body_part(&body_part_kind)
            .as_ref()
            .unwrap()
            .protection()
            .is_none()
    )
}