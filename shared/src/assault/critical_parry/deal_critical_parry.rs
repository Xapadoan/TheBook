use crate::random::Random;

use super::CriticalParry;

pub trait DealCriticalParry {
    fn deal_critical_parry(&self) -> CriticalParry {
        CriticalParry::random()
    }
}
