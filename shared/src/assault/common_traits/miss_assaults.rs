use crate::assault::assault_consequence::IndividualConsequences;
use crate::temporary_handicap::TemporaryHandicap;

pub trait ResolveMissAssaults {
    fn resolve_miss_assaults(&self, count: u8) -> IndividualConsequences {
        IndividualConsequences::miss_assaults(TemporaryHandicap::new(count))
    }
}