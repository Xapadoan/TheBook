use crate::warrior::assault::show_action::ShowAction;
use crate::warrior::body::HasBody;
use crate::warrior::temporary_handicap::assaults_miss::CanMissAssaults;
use crate::warrior::weapon::MayHaveWeapon;
use crate::warrior::{IsDead, IsUnconscious, HasName};

#[derive(Debug)]
pub enum CantBeAttackedReason {
    IsDead,
    IsUnconscious,
    HasNoWeapon,
}

impl ShowAction for CantBeAttackedReason {
    fn show<A, V>(&self, assailant: &A, victim: &V)
    where
        A: MayHaveWeapon + HasName + CanMissAssaults,
        V: HasName + HasBody
    {
        match self {
            CantBeAttackedReason::IsDead => println!("{} won't attack because {} is already dead", assailant.name(), victim.name()),
            CantBeAttackedReason::IsUnconscious => println!("{} won't attack because {} is unconscious", assailant.name(), victim.name()),
            CantBeAttackedReason::HasNoWeapon => println!("{} won't attack because {} has no weapon", assailant.name(), victim.name()),
        }
    }
}

#[derive(Debug)]
pub struct CanBeAttackedResult {
    can_be_attacked: bool,
    reason: Option<CantBeAttackedReason>
}

impl CanBeAttackedResult {
    pub fn can_be_attacked(&self) -> bool {
        self.can_be_attacked
    }

    pub fn reason(self) -> Option<CantBeAttackedReason> {
        self.reason
    }
}

pub trait CanBeAttacked {
    fn can_be_attacked(&self) -> CanBeAttackedResult;
}

impl<T: IsDead + IsUnconscious + MayHaveWeapon> CanBeAttacked for T {
    fn can_be_attacked(&self) -> CanBeAttackedResult {
        if self.weapon().is_none() {
            CanBeAttackedResult {
                can_be_attacked: false,
                reason: Some(CantBeAttackedReason::HasNoWeapon),
            }
        } else if self.is_dead() {
            CanBeAttackedResult {
                can_be_attacked: false,
                reason: Some(CantBeAttackedReason::IsDead),
            }
        } else if self.is_unconscious() {
            CanBeAttackedResult {
                can_be_attacked: false,
                reason: Some(CantBeAttackedReason::IsUnconscious)
            }
        } else {
            CanBeAttackedResult {
                can_be_attacked: true,
                reason: None,
            }
        }
    }
}
