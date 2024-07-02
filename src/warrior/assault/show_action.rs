use crate::warrior::body::HasBody;
use crate::warrior::temporary_handicap::parries_miss::CanMissParries;
use crate::warrior::weapon::MayHaveWeapon;
use crate::warrior::HasName;
use crate::warrior::CanMissAssaults;

pub trait ShowAction {
    fn show<A, V>(&self, assailant: &A, victim: &V)
    where
        A: HasName + MayHaveWeapon + CanMissAssaults,
        V: HasName + MayHaveWeapon + HasBody + CanMissParries;
}
