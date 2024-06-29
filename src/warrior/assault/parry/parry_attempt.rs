use crate::dice::{Dice, TestRollResult};

#[derive(Debug)]
pub enum ParryAttemptResult {
    CriticalFailure,
    Failure,
    Success,
    CriticalSuccess,
}

pub trait ParryThreshold {
    fn parry_threshold(&self) -> u8;
}

pub trait ParryAttempt {
    fn parry_attempt(&self) -> ParryAttemptResult;
}

impl<T: ParryThreshold> ParryAttempt for T {
    fn parry_attempt(&self) -> ParryAttemptResult {
        dbg!(self.parry_threshold());
        match Dice::D20.test_roll(self.parry_threshold()) {
            TestRollResult::CriticalSuccess => ParryAttemptResult::CriticalSuccess,
            TestRollResult::Success => ParryAttemptResult::Success,
            TestRollResult::Failure => ParryAttemptResult::Failure,
            TestRollResult::CriticalFailure => ParryAttemptResult::CriticalFailure
        }
    }
}