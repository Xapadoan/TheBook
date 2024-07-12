use crate::warrior::assault::damage_summary::DamageSummary;
use crate::warrior::assault::execute_action::ExecuteAction;
use crate::warrior::{HasName, IsDead, IsUnconscious, Warrior};
use crate::warrior::assault::show_action::ShowAction;
use crate::warrior::body::HasBody;
use crate::warrior::temporary_handicap::parries_miss::CanMissParries;
use crate::warrior::weapon::MayHaveWeapon;

#[derive(Debug)]
pub enum CantParryReason {
    NoWeapon,
    IsDead,
    IsUnconscious,
    MustMissParry,
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

impl<T: MayHaveWeapon + CanMissParries + IsDead + IsUnconscious> CanParry for T {
    fn can_parry(&self) -> CanParryResult {
        if self.is_dead() {
            return CanParryResult { can_parry: false, reason: Some(CantParryReason::IsDead) };
        }
        if self.is_unconscious() {
            return CanParryResult { can_parry: false, reason: Some(CantParryReason::IsUnconscious) };
        }
        match self.weapon() {
            Some(_) => if self.must_miss_parry() {
                CanParryResult { can_parry: false, reason: Some(CantParryReason::MustMissParry) }
            } else {
                CanParryResult { can_parry: true, reason: None }
            },
            None => CanParryResult { can_parry: false, reason: Some(CantParryReason::NoWeapon) },
        }
    }
}

impl ShowAction for CantParryReason {
    fn show<A, V>(&self, _: &A, victim: &V)
    where
        A: MayHaveWeapon + HasName,
        V: HasName + HasBody + CanMissParries
    {
        match self {
            CantParryReason::IsDead => println!("{} can't parry because he is dead", victim.name()),
            CantParryReason::IsUnconscious => println!("{} can't parry because he is unconscious", victim.name()),
            CantParryReason::MustMissParry => println!("{} can't parry because {}", victim.name(), victim.must_miss_parry_reason()),
            CantParryReason::NoWeapon => println!("{} can't attack attack he has no weapon", victim.name()),
        }
    }
}
impl ExecuteAction for CanParryResult {
    fn execute(&mut self, _: &mut Warrior, victim: &mut Warrior) -> DamageSummary {
        match &self.reason {
            Some(reason) => match reason {
                CantParryReason::MustMissParry => victim.miss_parry(),
                _ => {},
            },
            None => {},
        }
        DamageSummary::new(0)
    }
}
