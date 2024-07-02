use serde::{Deserialize, Serialize};

use crate::dice::{RollDamage, Dice};

#[derive(Debug, Serialize, Deserialize)]
pub struct DurationDamage {
    interval_sec: u32,
    reason: String,
    start_at: u32,
    hits_count: u32,
}

impl DurationDamage {
    pub fn new(reason: String, start_at: u32) -> Self {
        Self { reason, interval_sec: 60, start_at, hits_count: 0 }
    }

    pub fn should_take_duration_damage(&self, time_elapsed: u32) -> bool {
        let relative_time = time_elapsed - self.start_at;
        if relative_time == 0 {
            return false;
        }
        let should_have_been_hit_count = relative_time / self.interval_sec;
        should_have_been_hit_count > self.hits_count
    }

    pub fn add_hit(&mut self) {
        self.hits_count += 1;
    }

    pub fn reason(&self) -> &String {
        &self.reason
    }
}

impl RollDamage for DurationDamage {
    fn roll_damage(&self) -> u8 {
        Dice::D6.roll()
    }
}

pub trait MayHaveDurationDamage {
    fn add_duration_damage(&mut self, reason: String, start_at: u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_take_damage_when_needed() {
        let mut damage = DurationDamage::new(String::from("Test"), 34);
        assert!(
            !damage.should_take_duration_damage(34),
            "Should not take damage right away",
        );
        assert!(
            !damage.should_take_duration_damage(93),
            "Should not take damage until 60 secs passed",
        );
        assert!(
            damage.should_take_duration_damage(96),
            "Should take damage after 60 secs",
        );
        assert!(
            damage.should_take_duration_damage(97),
            "Should take damage again if still didn't inflict damage",
        );
        damage.add_hit();
        assert!(
            !damage.should_take_duration_damage(98),
            "Should not take damage again after inflicting hit"
        )
    }
}
