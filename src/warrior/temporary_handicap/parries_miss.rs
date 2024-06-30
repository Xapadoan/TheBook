use super::TemporaryHandicap;

pub trait CanMissParries {
    fn must_miss_parry(&self) -> bool;
    fn must_miss_parry_reason(&self) -> &String;
    fn will_miss_parries(&mut self, misses: ParriesMiss);
    fn miss_parry(&mut self);
}

#[derive(Debug)]
pub struct ParriesMiss {
    count: u8,
    reason: String,
}

impl ParriesMiss {
    pub fn new(count: u8, reason: String) -> Self {
        Self {
            count,
            reason,
        }
    }
}

impl TemporaryHandicap for ParriesMiss {
    fn decrement_turns_count(&mut self) {
        self.count -= 1;
    }

    fn turns_left(&self) -> u8 {
        self.count
    }

    fn reason(&self) -> &String {
        &self.reason
    }
}