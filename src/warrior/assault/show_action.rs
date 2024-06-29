use crate::warrior::body::HasBody;
use crate::warrior::temporary_handicap::parries_miss::CanMissParries;
use crate::warrior::weapon::MayHaveWeapon;
use crate::warrior::Name;
use crate::warrior::CanMissAssaults;

pub trait ShowAction {
    fn show<A, V>(&self, assailant: &A, victim: &V)
    where
        A: Name + MayHaveWeapon + CanMissAssaults,
        V: Name + MayHaveWeapon + HasBody + CanMissParries;
}
