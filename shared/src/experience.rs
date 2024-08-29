pub trait Experience {
    fn xp(&self) -> u64;
    fn level(&self) -> u8 {
        let xp = self.xp();
        let mut level = 1;
        let mut level_gap = 100;

        while xp >= level_gap {
            level_gap += 2 * level_gap;
            level += 1;
        }
        level
    }
}

pub trait GainExperience {
    fn gain_xp(&mut self, xp: u64);
}
