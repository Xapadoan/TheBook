use shared::{assault::assault_consequence::{AssaultConsequences, IndividualConsequences}, health::MutableHealth, inventory::Inventory};

use crate::common::TestAssailant;

#[test]
pub fn raw_damages_are_applied() {
    let mut assailant = TestAssailant::new();
    let mut victim = TestAssailant::new();
    let mut assailant_dropped_items = Inventory::new();
    let mut victim_dropped_items = Inventory::new();
    let consequences = AssaultConsequences::new(
        IndividualConsequences::only_raw_damages(5),
        IndividualConsequences::no_consequences(),
    );
    assert_eq!(assailant.health().current(), assailant.health().max());

    consequences.apply(
        &mut assailant,
        &mut assailant_dropped_items,
        &mut victim,
        &mut victim_dropped_items,
    );
    assert_eq!(assailant.health().current(), assailant.health().max() - 5);
}