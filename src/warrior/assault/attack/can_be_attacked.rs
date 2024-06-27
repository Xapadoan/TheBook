use crate::warrior::{weapon::MayHaveWeapon, IsDead, IsUnconscious};

#[derive(Debug)]
pub enum CantBeAttackedReason {
    IsDead,
    IsUnconscious,
    HasNoWeapon,
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
