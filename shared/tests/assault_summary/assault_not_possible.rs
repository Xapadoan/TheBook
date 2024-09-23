use crate::common::TestAssailant;
use shared::assault::assault_consequence::{AssaultConsequences, IndividualConsequences};
use shared::assault::assault_summary::AssaultSummary;
use shared::equipment::rupture::RUPTURE_MAX;
use shared::equipment::weapon::{OptionalMutableWeapon, Weapon};
use shared::random::Random;
use shared::temporary_handicap::{OptionalAssaultMisses, TemporaryHandicap, TemporaryHandicapReason};

#[test]
fn cant_assault_without_weapon() {
    let assailant = TestAssailant::new();
    let victim = TestAssailant::new();
    assert_eq!(assailant.weapon().is_none(), true, "Assailant has a weapon");

    let assault = AssaultSummary::new(&assailant, &victim);
    assert_eq!(assault.not_possible().is_some(), true, "Assault is still possible");
}

#[test]
fn cant_assault_after_losing_weapon() {
    let mut warrior1 = TestAssailant::new();
    let mut warrior2 = TestAssailant::new();
    let weapon1 = Weapon::random();
    let weapon2 = Weapon::random();
    warrior1.weapon_mut().replace(weapon1);
    warrior2.weapon_mut().replace(weapon2);

    let victim_loses_weapon = AssaultConsequences::new(
        IndividualConsequences::no_consequences(),
        IndividualConsequences::damage_weapon(RUPTURE_MAX),
    );
    victim_loses_weapon.apply(
        &mut warrior1,
        &mut warrior2,
    );
    assert_eq!(warrior2.weapon().is_none(), true, "Weapon is still Some");

    let after = AssaultSummary::new(&warrior2, &warrior1);
    assert_eq!(after.not_possible().is_some(), true, "Assault is still possible");
}

#[test]
fn cant_assault_when_misses_assaults() {
    let misses = TemporaryHandicap::new(
        2,
        TemporaryHandicapReason::FellDown,
    );
    let mut assailant = TestAssailant::new();
    let mut victim = TestAssailant::new();
    let weapon1 = Weapon::random();
    assailant.weapon_mut().replace(weapon1);
    let weapon2 = Weapon::random();
    victim.replace_weapon(weapon2);
    let consequences = AssaultConsequences::new(
        IndividualConsequences::unstoppable_assaults(misses.clone()),
        IndividualConsequences::no_consequences(),
    );

    consequences.apply(
        &mut assailant,
        &mut victim,
    );
    assert!(assailant.assault_misses().is_some(), "can still attack");

    let assault = AssaultSummary::new(&mut assailant, &mut victim);
    assert!(assault.not_possible().is_some(), "Assault is still possible");
}

#[test]
fn when_assault_is_not_possible_nothing_happens() {
    let assailant = TestAssailant::new();
    let victim = TestAssailant::new();
    let assault = AssaultSummary::new(&assailant, &victim);
    assert_eq!(assault.not_possible().is_some(), true, "Assault is possible");
    assert_eq!(assault.attack_clumsiness().is_none(), true, "AttackClumsiness is Some");
    assert_eq!(assault.attack_critical().is_none(), true, "AttackSuccess is Some");
    assert_eq!(assault.attack_missed().is_none(), true, "AttackMissed is Some");
    assert_eq!(assault.attack_success().is_none(), true, "AttackSuccess is Some");
    assert_eq!(assault.parry_clumsiness().is_none(), true, "ParryClumsiness is Some");
    assert_eq!(assault.parry_critical().is_none(), true, "ParryCritical is Some");
    assert_eq!(assault.parry_success().is_none(), true, "ParryCritical is Some");
}
