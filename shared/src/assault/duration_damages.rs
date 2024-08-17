use serde::{Deserialize, Serialize};

use crate::dice::Dice;

use super::common_traits::DealDamages;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DurationDamages {
    interval: u8,
    count_remaining: u8,
}

pub trait TakeDurationDamages {
    fn duration_damages(&self) -> &Vec<DurationDamages>;
    fn duration_damages_mut(&mut self) -> &mut Vec<DurationDamages>;
    fn take_duration_damages(&mut self) -> Option<u8> {
        if self.duration_damages().len() < 1 {
            return None;
        }
        let mut total_damages = 0;
        for duration_damage in self.duration_damages_mut() {
            if let Some(damages) = duration_damage.apply() {
                total_damages += damages;
            }
        }
        Some(total_damages)
    }
}

impl DurationDamages {
    pub fn new() -> Self {
        Self { interval: 4, count_remaining: 4 }
    }

    pub fn apply(&mut self) -> Option<u8> {
        self.count_remaining -= 1;
        if self.count_remaining == 0 {
            self.count_remaining = self.interval;
            return Some(self.deal_damages());
        }
        return None;
    }
}

impl DealDamages for DurationDamages {
    fn deal_damages(&self) -> u8 {
        Dice::D6.roll()
    }
}
