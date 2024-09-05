use chrono::{DateTime, TimeDelta, Utc};

use super::MutableHealth;

const PASSIVE_HEAL_INTERVAL: u64 = 3600;
const PASSIVE_HEAL_RATIO: u64 = 2;

pub trait PassiveHealing {
    fn last_passive_heal(&self) -> DateTime<Utc>;
}

pub trait MutablePassiveHealing:
    PassiveHealing +
    MutableHealth
{
    fn set_last_passive_heal(&mut self, last_passive_heal: DateTime<Utc>);
    fn passive_heal(&mut self) {
        let rest_duration = (Utc::now() - self.last_passive_heal()).num_seconds() as u64;
        let intervals_passed = rest_duration / PASSIVE_HEAL_INTERVAL;
        let heal_amount = intervals_passed * PASSIVE_HEAL_RATIO;
        let current_health = self.health().current();
        if heal_amount > u8::MAX as u64 {
            self.health_mut().set(u8::MAX)
        } else {
            self.health_mut().set(current_health + heal_amount as u8)
        }
        let updated_rest_duration = TimeDelta::seconds((rest_duration % PASSIVE_HEAL_INTERVAL) as i64);
        self.set_last_passive_heal(Utc::now() - updated_rest_duration)
    }
}