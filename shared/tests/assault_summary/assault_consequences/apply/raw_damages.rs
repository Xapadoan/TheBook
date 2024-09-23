use shared::assault::assault_consequence::{AssaultConsequences, IndividualConsequences};
use shared::health::MutableHealth;

use crate::common::TestAssailant;

#[test]
pub fn raw_damages_are_applied() {
    let mut assailant = TestAssailant::new();
    let mut victim = TestAssailant::new();
    let consequences = AssaultConsequences::new(
        IndividualConsequences::only_raw_damages(5),
        IndividualConsequences::no_consequences(),
    );
    assert_eq!(assailant.health().current(), assailant.health().max());

    consequences.apply(
        &mut assailant,
        &mut victim,
    );
    assert_eq!(assailant.health().current(), assailant.health().max() - 5);
}