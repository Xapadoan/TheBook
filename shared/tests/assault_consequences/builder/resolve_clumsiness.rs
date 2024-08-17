use shared::{assault::{attack_clumsiness::ResolveAttackClumsiness, clumsiness::Clumsiness, parry_clumsiness::ResolveParryClumsiness}, equipment::weapon::{OptionalMutableWeapon, Weapon}, random::Random};

use crate::common::TestAssailant;

#[test]
pub fn test_resolve_self_critical_hit() {
    let mut assailant = TestAssailant::new();
    let weapon = Weapon::random();
    assailant.weapon_mut().replace(weapon);
    let consequences = assailant.resolve_attack_clumsiness(Clumsiness::CriticalHitSelf);
    assert_eq!(consequences.self_critical_hit().is_some(), true, "self_critical_hit is not Some");
    let consequences = assailant.resolve_parry_clumsiness(Clumsiness::CriticalHitSelf, 0);
    assert_eq!(consequences.self_critical_hit().is_some(), true, "self_critical_hit is not Some");
}
