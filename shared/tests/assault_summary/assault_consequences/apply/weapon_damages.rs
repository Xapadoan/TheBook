use crate::common::TestAssailant;
use shared::assault::assault_consequence::{AssaultConsequences, IndividualConsequences};
use shared::equipment::rupture::{Rupture, RUPTURE_MAX};
use shared::equipment::weapon::{OptionalMutableWeapon, Weapon};
use shared::inventory::Inventory;
use shared::random::Random;

#[test]
pub fn assailant_drops_weapon_when_broken() {
    let consequences = AssaultConsequences::new(
        IndividualConsequences::damage_weapon(RUPTURE_MAX),
        IndividualConsequences::no_consequences(),
    );
    let mut assailant = TestAssailant::new();
    let mut victim = TestAssailant::new();
    let mut assailant_dropped_items = Inventory::new();
    let mut victim_dropped_items = Inventory::new();
    let weapon = Weapon::random();
    assailant.weapon_mut().replace(weapon);
    assert_eq!(assailant.weapon().is_some(), true, "Assailant has no weapon");

    consequences.apply(
        &mut assailant,
        &mut assailant_dropped_items,
        &mut victim,
        &mut victim_dropped_items,
    );
    assert_eq!(assailant.weapon().is_none(), true, "Assailant didn't lose his weapon");
}

#[test]
pub fn victim_drops_weapon_when_broken() {
    let consequences = AssaultConsequences::new(
        IndividualConsequences::no_consequences(),
        IndividualConsequences::damage_weapon(RUPTURE_MAX),
    );
    let mut assailant = TestAssailant::new();
    let mut victim = TestAssailant::new();
    let mut assailant_dropped_items = Inventory::new();
    let mut victim_dropped_items = Inventory::new();
    let weapon = Weapon::random();
    victim.weapon_mut().replace(weapon);
    assert!(victim.weapon().is_some(), "Victim has no weapon");

    consequences.apply(
        &mut assailant,
        &mut assailant_dropped_items,
        &mut victim,
        &mut victim_dropped_items,
    );
    assert!(victim.weapon().is_none(), "Victim didn't lose his weapon");
}

#[test]
pub fn assailant_does_not_drop_weapon_if_only_damaged() {
    let consequences = AssaultConsequences::new(
        IndividualConsequences::no_consequences(),
        IndividualConsequences::damage_weapon(1),
    );
    let mut assailant = TestAssailant::new();
    let mut victim = TestAssailant::new();
    let mut assailant_dropped_items = Inventory::new();
    let mut victim_dropped_items = Inventory::new();
    let weapon = Weapon::random();
    let weapon_rupture = *weapon.rupture().as_ref().unwrap();
    assailant.weapon_mut().replace(weapon);
    assert!(assailant.weapon().is_some(), "Assailant has no weapon");
    
    consequences.apply(
        &mut assailant,
        &mut assailant_dropped_items,
        &mut victim,
        &mut victim_dropped_items,
    );
    assert!(assailant.weapon().is_some(), "Assailant dropped his weapon");
    let weapon = assailant.weapon().as_ref().unwrap();
    assert_eq!(*weapon.rupture().as_ref().unwrap(), weapon_rupture);
}

#[test]
pub fn broken_weapon_is_not_in_dropped_items() {
    let consequences = AssaultConsequences::new(
        IndividualConsequences::no_consequences(),
        IndividualConsequences::damage_weapon(RUPTURE_MAX),
    );
    let mut assailant = TestAssailant::new();
    let mut victim = TestAssailant::new();
    let mut assailant_dropped_items = Inventory::new();
    let mut victim_dropped_items = Inventory::new();
    let weapon = Weapon::random();
    victim.weapon_mut().replace(weapon);
    assert!(victim.weapon().is_some(), "Victim has no weapon");

    consequences.apply(
        &mut assailant,
        &mut assailant_dropped_items,
        &mut victim,
        &mut victim_dropped_items,
    );
    assert_eq!(victim_dropped_items.items().len(), 0, "Victim's weapon is in its dropped items");
}
