use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TemporaryHandicapReason {
    LostBalance,
    FellDown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemporaryHandicap {
    count: u8,
    reason: TemporaryHandicapReason,
}

impl TemporaryHandicap {
    pub fn new(count: u8, reason: TemporaryHandicapReason) -> Self {
        Self { count, reason }
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    pub fn reason(&self) -> &TemporaryHandicapReason {
        &self.reason
    }

    fn decrease_count(&mut self) {
        if self.count > 0 {
            self.count -= 1;
        }
    }
}

pub trait OptionalAssaultMisses {
    fn assault_misses(&self) -> &Option<TemporaryHandicap>;
}
pub trait OptionalMutableAssaultMisses:
    OptionalAssaultMisses
{
    fn assault_misses_mut(&mut self) -> &mut Option<TemporaryHandicap>;
    fn miss_assault(&mut self) {
        if let Some(misses) = self.assault_misses_mut() {
            misses.decrease_count();
            if misses.count < 1 {
                self.assault_misses_mut().take();
            }
        }
    }
}

pub trait OptionalParryMisses {
    fn parry_misses(&self) -> &Option<TemporaryHandicap>;
}

pub trait OptionalMutableParryMisses:
    OptionalAssaultMisses
{
    fn parry_misses_mut(&mut self) -> &mut Option<TemporaryHandicap>;
    fn miss_parry(&mut self) {
        if let Some(misses) = self.parry_misses_mut() {
            misses.decrease_count();
            if misses.count < 1 {
                self.parry_misses_mut().take();
            }
        }
    }
}
