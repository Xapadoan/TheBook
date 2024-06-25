use crate::dice::Dice;

use super::RollDamage;

#[derive(Debug)]
pub struct DurationDamage {
    interval_sec: u32,
    reason: String,
    start_at: u32,
}

impl DurationDamage {
    pub fn new(reason: String, start_at: u32) -> Self {
        Self { reason, interval_sec: 60, start_at }
    }

    pub fn should_take_duration_damage(&self, time_elapsed: u32) -> bool {
        let relative_time = time_elapsed - self.start_at;
        relative_time % self.interval_sec == 0
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
