use crate::common::TestAssailant;
use shared::assault::assault_consequence::{AssaultConsequences, IndividualConsequences};
use shared::equipment::rupture::RUPTURE_MAX;
use shared::equipment::weapon::{OptionalMutableWeapon, Weapon};
use shared::random::Random;

#[test]
pub fn test_weapon_is_dropped_when_broken() {
    let consequences = AssaultConsequences::new(
        IndividualConsequences::damage_weapon(RUPTURE_MAX),
        IndividualConsequences::no_consequences(),
    );
    let mut assailant = TestAssailant::new();
    let mut victim = TestAssailant::new();
    let weapon = Weapon::random();
    assailant.weapon_mut().replace(weapon);
    assert_eq!(assailant.weapon().is_some(), true, "Assailant has no weapon");
    consequences.apply(&mut assailant, &mut victim);
    assert_eq!(assailant.weapon().is_none(), true, "Assailant didn't lose his weapon");
}
