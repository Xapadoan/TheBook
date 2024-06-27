use rand::Rng;

pub enum Dice {
    D20,
    D6,
}

impl Dice {
    pub fn test_roll(&self, success_threshold: u8) -> TestRollResult {
        let face = self.roll();
        if face == 1 {
            TestRollResult::CriticalSuccess
        } else if face == 20 {
            TestRollResult::CriticalFailure
        } else if face > success_threshold {
            TestRollResult::Failure
        } else {
            TestRollResult::Success
        }
    }

    pub fn roll(&self) -> u8 {
        let rng = match self {
            Dice::D20 => 1..20,
            Dice::D6 => 1..6,
        };
        rand::thread_rng().gen_range(rng)
    }
}

pub enum TestRollResult {
    CriticalSuccess,
    Success,
    Failure,
    CriticalFailure,
}
