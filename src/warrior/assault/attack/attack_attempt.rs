use crate::dice::{Dice, TestRollResult};

#[derive(Debug)]
pub enum AttackAttemptResult {
    CriticalFailure,
    Failure,
    Success,
    CriticalSuccess,
}

pub trait AttackThreshold {
    fn attack_threshold(&self) -> u8;
}

pub trait AttackAttempt {
    fn attack_attempt(&self) -> AttackAttemptResult;
}

impl<T: AttackThreshold> AttackAttempt for T {
    fn attack_attempt(&self) -> AttackAttemptResult {
        match Dice::D6.test_roll(self.attack_threshold()) {
            TestRollResult::CriticalSuccess => AttackAttemptResult::CriticalSuccess,
            TestRollResult::Success => AttackAttemptResult::Success,
            TestRollResult::Failure => AttackAttemptResult::Failure,
            TestRollResult::CriticalFailure => AttackAttemptResult::CriticalFailure
        }
    }
}
