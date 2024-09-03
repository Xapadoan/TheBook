use shared::tournament::Tournament;

pub trait FightReward {
    fn fight_reward(&self, round_index: usize) -> u32;
}

impl FightReward for Tournament {
    fn fight_reward(&self, round_index: usize) -> u32 {
        let total_rounds = self.number_of_rounds();
        if round_index + 1 < total_rounds / 2 {
            return 0;
        } else if round_index + 1 < total_rounds * 3 / 4 {
            return 5;
        } else if round_index == total_rounds - 2 {
            return 10;
        } else {
            return 20;
        }
    }
}
