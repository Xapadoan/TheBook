use crate::assault::assault_consequence::IndividualConsequences;

pub trait ResolveDropWeapon {
    fn resolve_drop_weapon(&self) -> IndividualConsequences {
        IndividualConsequences::drop_weapon()
    }
}