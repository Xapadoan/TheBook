use shared::assault::assault_consequence::{AssaultConsequences, IndividualConsequences};
use shared::equipment::weapon::{OptionalMutableWeapon, Weapon};
use shared::random::Random;
use shared::temporary_handicap::{
    OptionalAssaultMisses,
    OptionalMutableAssaultMisses,
    OptionalMutableParryMisses,
    OptionalParryMisses,
    TemporaryHandicap,
    TemporaryHandicapReason,
};

use crate::common::TestAssailant;

#[test]
fn victim_miss_parry_if_not_just_applied() {
    let mut assailant = TestAssailant::new();
    let weapon = Weapon::random();
    assailant.weapon_mut().replace(weapon);

    let mut victim = TestAssailant::new();
    let weapon = Weapon::random();
    victim.weapon_mut().replace(weapon);
    victim.parry_misses_mut().replace(
        TemporaryHandicap::new(2, TemporaryHandicapReason::FellDown)
    );
    assert_eq!(
        victim.parry_misses().as_ref().unwrap().count(),
        2,
        "victim doesn't already have misses",
    );

    let no_consequences = AssaultConsequences::new(
        IndividualConsequences::no_consequences(),
        IndividualConsequences::no_consequences(),
    );
    no_consequences.apply(
        &mut assailant,
        &mut victim,
    );
    assert_eq!(
        victim.parry_misses().as_ref().unwrap().count(),
        1,
        "victim's parry misses didn't decrease as expected",
    );
}

#[test]
fn victim_do_not_miss_parry_if_just_applied() {
    let mut assailant = TestAssailant::new();
    let weapon = Weapon::random();
    assailant.weapon_mut().replace(weapon);

    let mut victim = TestAssailant::new();
    let weapon = Weapon::random();
    victim.weapon_mut().replace(weapon);
    assert!(
        victim.parry_misses().is_none(),
        "victim already has misses",
    );

    let no_consequences = AssaultConsequences::new(
        IndividualConsequences::no_consequences(),
        IndividualConsequences::unstoppable_assaults(
            TemporaryHandicap::new(2, TemporaryHandicapReason::FellDown)
        ),
    );
    no_consequences.apply(
        &mut assailant,
        &mut victim,
    );
    assert_eq!(
        victim.parry_misses().as_ref().unwrap().count(),
        2,
        "victim's parry misses decreased",
    );
}

#[test]
fn assailant_miss_assault_if_not_just_applied() {
    let mut assailant = TestAssailant::new();
    let weapon = Weapon::random();
    assailant.weapon_mut().replace(weapon);
    assailant.assault_misses_mut().replace(
        TemporaryHandicap::new(2, TemporaryHandicapReason::FellDown)
    );
    assert_eq!(
        assailant.assault_misses().as_ref().unwrap().count(),
        2,
        "assailant doesn't already have misses",
    );

    let mut victim = TestAssailant::new();
    let weapon = Weapon::random();
    victim.weapon_mut().replace(weapon);

    let no_consequences = AssaultConsequences::new(
        IndividualConsequences::no_consequences(),
        IndividualConsequences::no_consequences(),
    );
    no_consequences.apply(
        &mut assailant,
        &mut victim,
    );
    assert_eq!(
        assailant.assault_misses().as_ref().unwrap().count(),
        1,
        "assailant's assault misses didn't decrease as expected",
    );
}

#[test]
fn assailant_do_not_miss_assault_if_just_applied() {
    let mut assailant = TestAssailant::new();
    let weapon = Weapon::random();
    assailant.weapon_mut().replace(weapon);
    assert!(
        assailant.assault_misses().is_none(),
        "assailant already has misses",
    );

    let mut victim = TestAssailant::new();
    let weapon = Weapon::random();
    victim.weapon_mut().replace(weapon);

    let no_consequences = AssaultConsequences::new(
        IndividualConsequences::miss_assaults(
            TemporaryHandicap::new(2, TemporaryHandicapReason::FellDown)
        ),
        IndividualConsequences::no_consequences(),
    );
    no_consequences.apply(
        &mut assailant,
        &mut victim,
    );
    assert_eq!(
        assailant.assault_misses().as_ref().unwrap().count(),
        2,
        "assailant's assault misses decreased",
    );
}
