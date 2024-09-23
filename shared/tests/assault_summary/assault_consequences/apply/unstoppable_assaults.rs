use shared::assault::assault_consequence::{AssaultConsequences, IndividualConsequences};
use shared::temporary_handicap::{
    OptionalAssaultMisses,
    OptionalParryMisses,
    TemporaryHandicap,
    TemporaryHandicapReason,
};

use crate::common::TestAssailant;

#[test]
pub fn unstoppable_assaults_causes_parry_misses() {
    let misses = TemporaryHandicap::new(
        2,
        TemporaryHandicapReason::FellDown,
    );
    let mut assailant = TestAssailant::new();
    let mut victim = TestAssailant::new();
    let consequences = AssaultConsequences::new(
        IndividualConsequences::unstoppable_assaults(misses.clone()),
        IndividualConsequences::no_consequences(),
    );

    consequences.apply(
        &mut assailant,
        &mut victim,
    );
    assert!(assailant.parry_misses().is_some(), "can still parry");

    let actual_misses = assailant.parry_misses().as_ref().unwrap();
    assert_eq!(actual_misses.count(), misses.count(), "misses count don't match");
    assert_eq!(actual_misses.reason(), misses.reason(), "misses reason don't match");
}

#[test]
pub fn unstoppable_assaults_causes_assault_misses() {
    let misses = TemporaryHandicap::new(
        2,
        TemporaryHandicapReason::FellDown,
    );
    let mut assailant = TestAssailant::new();
    let mut victim = TestAssailant::new();
    let consequences = AssaultConsequences::new(
        IndividualConsequences::unstoppable_assaults(misses.clone()),
        IndividualConsequences::no_consequences(),
    );

    consequences.apply(
        &mut assailant,
        &mut victim,
    );
    assert!(assailant.assault_misses().is_some(), "can still attack");

    let actual_misses = assailant.parry_misses().as_ref().unwrap();
    assert_eq!(actual_misses.count(), misses.count(), "misses count don't match");
    assert_eq!(actual_misses.reason(), misses.reason(), "misses reason don't match");
}
