use crate::assault::assailant::Assailant;
use crate::assault::assault_consequence::IndividualConsequences;
use crate::assault::common_traits::ResolveBreakWeapon;
use crate::assault::common_traits::ResolveDropWeapon;
use crate::assault::common_traits::ResolveMissAssaults;
use crate::assault::critical_hit::DealCriticalHit;
use crate::assault::critical_hit::{ResolveCriticalHit, ResolveCriticalHitSelf};
use crate::assault::common_traits::DealDamages;
use crate::assault::common_traits::ReduceDamages;
use crate::temporary_handicap::TemporaryHandicap;
use crate::temporary_handicap::TemporaryHandicapReason;

use super::CriticalParry;

pub trait ResolveCriticalParry:
    ReduceDamages +
    DealCriticalHit +
    DealDamages +
    ResolveMissAssaults +
    ResolveDropWeapon +
    ResolveBreakWeapon +
    ResolveCriticalHit +
    ResolveCriticalHitSelf
{
    fn resolve_critical_parry(
        &self, critical_parry: &CriticalParry,
        parry_author: &dyn Assailant,
    ) -> IndividualConsequences {
        match critical_parry {
            CriticalParry::RegularParry => IndividualConsequences::no_consequences(),
            CriticalParry::AssailantRepelled => self.resolve_miss_assaults(
                TemporaryHandicap::new(1, TemporaryHandicapReason::LostBalance)
            ),
            CriticalParry::AssailantTrips => self.resolve_unstoppable_attack(
                TemporaryHandicap::new(1, TemporaryHandicapReason::LostBalance)
            ),
            CriticalParry::AssailantFalls => self.resolve_unstoppable_attack(
                TemporaryHandicap::new(2, TemporaryHandicapReason::FellDown)
            ),
            CriticalParry::AssailantDropsWeapon => self.resolve_drop_weapon(),
            CriticalParry::AssailantBreaksWeapon => self.resolve_break_weapon(),
            CriticalParry::AssailantHit => self.resolve_counter_hit(parry_author),
            CriticalParry::AssailantCriticalHit => self.resolve_counter_critical_hit(parry_author),
            CriticalParry::AssailantSelfCriticalHit => self.resolve_critical_hit_self(),
        }
    }
    fn resolve_unstoppable_attack(&self, misses: TemporaryHandicap) -> IndividualConsequences {
        IndividualConsequences::unstoppable_assaults(misses)
    }
    fn resolve_counter_hit(&self, parry_author: &dyn Assailant) -> IndividualConsequences {
        IndividualConsequences::only_damages(self.reduce_damages(parry_author.deal_damages()))
    }
    fn resolve_counter_critical_hit(&self, parry_author: &dyn Assailant) -> IndividualConsequences {
        let critical_hit = parry_author.deal_critical_hit();
        let damages = parry_author.deal_damages();
        let mut consequence = self.resolve_critical_hit(damages, &critical_hit);
        consequence.add_counter_critical_hit(critical_hit);
        consequence
    }
}
