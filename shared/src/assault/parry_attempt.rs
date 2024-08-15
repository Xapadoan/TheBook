use serde::{Deserialize, Serialize};

use crate::dice::{Dice, TestRollResult};

#[derive(Debug, Serialize, Deserialize)]
pub enum ParryAttemptResult {
    CriticalFailure,
    Failure,
    Success,
    CriticalSuccess,
}

pub trait ParryThreshold {
    fn parry_threshold(&self) -> u8;
}

pub trait ParryAttempt: ParryThreshold {
    fn parry_attempt(&self) -> ParryAttemptResult {
        match Dice::D20.test_roll(self.parry_threshold()) {
            TestRollResult::CriticalSuccess => ParryAttemptResult::CriticalSuccess,
            TestRollResult::Success => ParryAttemptResult::Success,
            TestRollResult::Failure => ParryAttemptResult::Failure,
            TestRollResult::CriticalFailure => ParryAttemptResult::CriticalFailure
        }
    }
}
