use shared::{assault::assault_consequence::{AssaultConsequences, IndividualConsequences}, equipment::protection::{OptionalMutableProtection, Protection, ProtectionKind}, inventory::{HasInventory, Item}, name::Name, warrior::body::{body_part::{BodyPartKind, BodySide, OptionalMutableBodyPart}, injury::Injury, HasMutableBody}};

use crate::common::TestAssailant;

#[test]
fn gloves_should_be_transferred_to_inventory() {
    let mut assailant = TestAssailant::new();
    let mut victim = TestAssailant::new();

    let affected_side = BodySide::Left;
    let protected_hand = BodyPartKind::Hand(affected_side.clone());

    let glove = Protection::new(ProtectionKind::Gloves);
    let glove_name = glove.name().to_string();
    victim.body_mut()
        .body_part_mut(&protected_hand)
        .as_mut()
        .unwrap()
        .replace_protection(glove);

    let consequences = AssaultConsequences::new(
        IndividualConsequences::no_consequences(),
        IndividualConsequences::injures(0, Injury::LeftArmSevered),
    );

    consequences.apply(&mut assailant, &mut victim);

    assert!(victim.inventory().items().len() > 0);
    let mut glove_found = false;
    for (_, item) in victim.inventory().items().iter() {
        match &item {
            Item::Protection(protection) => {
                if protection.name().to_string() == glove_name {
                    glove_found = true;
                }
            }
            _ => {},
        }
    }
    assert!(glove_found);
}