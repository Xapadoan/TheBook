use crate::dice::{Dice, TestRollResult};

pub enum AttackAttemptResult {
    CriticalFailure,
    Failure,
    Success,
    CriticalSuccess,
}

pub trait AttackThreshold {
    fn attack_threshold(&self) -> u8;
}

pub trait AttackAttempt: AttackThreshold {
    fn attack_attempt(&self) -> AttackAttemptResult {
        match Dice::D20.test_roll(self.attack_threshold()) {
            TestRollResult::CriticalSuccess => AttackAttemptResult::CriticalSuccess,
            TestRollResult::Success => AttackAttemptResult::Success,
            TestRollResult::Failure => AttackAttemptResult::Failure,
            TestRollResult::CriticalFailure => AttackAttemptResult::CriticalFailure
        }
    }
}
