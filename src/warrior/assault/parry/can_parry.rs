use crate::warrior::{assault::show_action::ShowAction, body::HasBody, temporary_handicap::assaults_miss::CanMissAssaults, weapon::MayHaveWeapon, Name};

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

    pub fn reason(&self) -> Option<&CantParryReason> {
        self.reason.as_ref()
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

impl ShowAction for CantParryReason {
    fn show<A, V>(&self, assailant: &A, victim: &V)
    where
        A: MayHaveWeapon + Name + CanMissAssaults,
        V: Name + HasBody
    {
        match self {
            CantParryReason::NoHit => println!("{} wont't parry because {} missed", victim.name(), assailant.name()),
            CantParryReason::NoWeapon => println!("{} can't parry because he has no weapon", victim.name())
        }
    }
}