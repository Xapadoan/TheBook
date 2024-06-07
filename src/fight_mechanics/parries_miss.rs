use super::TemporaryHandicap;

#[derive(Debug)]
pub struct ParriesMiss {
    count: u8,
    reason: String,
}

impl TemporaryHandicap for ParriesMiss {
    fn new(count: u8, reason: String) -> Self {
        Self {
            count,
            reason,
        }
    }

    fn decrement_count(&mut self) {
        self.count -= 1;
    }

    fn count(&self) -> u8 {
        self.count
    }

    fn reason(&self) -> &String {
        &self.reason
    }
}