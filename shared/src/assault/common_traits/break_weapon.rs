use crate::assault::assault_consequence::IndividualConsequences;
use crate::equipment::rupture::{Rupture, RuptureTestResult};
use crate::equipment::weapon::OptionalMutableWeapon;

pub trait ResolveBreakWeapon:
    OptionalMutableWeapon
{
    fn resolve_break_weapon(&self) -> IndividualConsequences {
        match self.weapon() {
            Some(weapon) => match weapon.rupture_test() {
                RuptureTestResult::Success => IndividualConsequences::damage_weapon(1),
                RuptureTestResult::Fail => IndividualConsequences::damage_weapon(u8::MAX),
            },
            None => IndividualConsequences::no_consequences()
        }
    }
}