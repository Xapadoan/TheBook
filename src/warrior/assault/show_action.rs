use crate::warrior::body::HasBody;
use crate::warrior::weapon::MayHaveWeapon;
use crate::warrior::Name;
use crate::warrior::CanMissAssaults;

pub trait ShowAction {
    fn show<A, V>(&self, assailant: &A, victim: &V)
    where
        A: MayHaveWeapon + Name + CanMissAssaults,
        V: Name + HasBody;
}
