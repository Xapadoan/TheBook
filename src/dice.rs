use rand::Rng;

pub trait RollDamage {
    fn roll_damage(&self) -> u8;
}

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
            Dice::D20 => 1..=20,
            Dice::D6 => 1..=6,
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

#[cfg(test)]
mod tests {
    use std::collections::{hash_map::Entry, HashMap};

    use super::*;

    fn gen_map(max: u8) -> HashMap<u8, bool> {
        let mut i = 1;
        let mut map = HashMap::new();
        while i <= max {
            map.insert(i, false);
            i += 1;
        }
        map
    }

    fn assert_roll_distribution(dice: Dice) {
        let (number_of_rounds, map_max, probability) = match dice {
            Dice::D20 => (150, 20, "0.05%"),
            Dice::D6 => (50, 6, "0.02%"),
        };
        let mut map = gen_map(map_max);
        let mut i = 0;
        while i < number_of_rounds {
            let result = dice.roll();
            match map.entry(result) {
                Entry::Occupied(_) => map.insert(result, true),
                Entry::Vacant(_) => panic!("D6 roll resulted in {result}")
            };
            i += 1;
        }
        let dice_results: Vec<bool> = map.into_values().collect();
        let expected_results = vec![true; map_max.into()];
        assert_eq!(
            dice_results,
            expected_results,
            "At least one roll possibility did not occur the probabilities for this to happen in a correct implementation are below {}, you may want to check.",
            probability,
        )
    }

    #[test]
    fn test_roll_distribution() {
        assert_roll_distribution(Dice::D6);
        assert_roll_distribution(Dice::D20);
    }
}