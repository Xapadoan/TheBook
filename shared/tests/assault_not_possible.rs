use common::TestAssailant;
use shared::assault::assault_summary::AssaultSummary;
use shared::equipment::weapon::OptionalMutableWeapon;

mod common;

#[test]
fn cant_assault_without_weapon() {
    let assailant = TestAssailant::new();
    let victim = TestAssailant::new();
    assert_eq!(assailant.weapon().is_none(), true, "Assailant has a weapon");
    let assault = AssaultSummary::new(&assailant, &victim);
    assert_eq!(assault.not_possible().is_some(), true, "Not Possible is not Some");
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
