use super::TemporaryHandicap;

#[derive(Debug)]
pub struct AssaultsMiss {
    reason: String,
    count: u8,
}

impl TemporaryHandicap for AssaultsMiss {
    fn new(count: u8, reason: String) -> Self {
        Self {
            count,
            reason,
        }
    }

    fn decrement_count(&mut self) {
        self.count -= 1;
    }

    fn reason(&self) -> &String {
        &self.reason
    }

    fn count(&self) -> u8 {
        self.count
    }
}