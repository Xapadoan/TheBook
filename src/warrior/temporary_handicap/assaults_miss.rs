use super::TemporaryHandicap;

pub trait CanMissAssaults {
    fn must_miss_assault(&self) -> bool;
    fn must_miss_assault_reason(&self) -> &String;
    fn will_miss_assault(&mut self, misses: AssaultsMiss);
    fn miss_assault(&mut self);
}

#[derive(Debug)]
pub struct AssaultsMiss {
    reason: String,
    count: u8,
}

impl AssaultsMiss {
    pub fn new(count: u8, reason: String) -> Self {
        Self {
            count,
            reason,
        }
    }
}

impl TemporaryHandicap for AssaultsMiss {
    fn decrement_turns_count(&mut self) {
        self.count -= 1;
    }

    fn reason(&self) -> &String {
        &self.reason
    }

    fn turns_left(&self) -> u8 {
        self.count
    }
}