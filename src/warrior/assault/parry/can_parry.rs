use crate::warrior::weapon::MayHaveWeapon;

#[derive(Debug)]
pub enum CantParryReason {
    NoWeapon,
    NoHit,
}

#[derive(Debug)]
pub struct CanParryResult {
    can_parry: bool,
    reason: Option<CantParryReason>
}

impl CanParryResult {
    pub fn can_parry(&self) -> bool {
        self.can_parry
    }

    pub fn reason(self) -> Option<CantParryReason> {
        self.reason
    }
}

pub trait CanParry {
    fn can_parry(&self) -> CanParryResult;
}

impl<T: MayHaveWeapon> CanParry for T {
    fn can_parry(&self) -> CanParryResult {
        match self.weapon() {
            Some(_) => CanParryResult { can_parry: true, reason: None },
            None => CanParryResult { can_parry: false, reason: Some(CantParryReason::NoWeapon) },
        }
    }
}