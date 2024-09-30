use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Health {
    max: u8,
    current: u8,
}

impl Health {
    pub fn new(max: u8, current: u8) -> Self {
        Self { max, current }
    }

    pub fn max(&self) -> u8 {
        self.max
    }

    pub fn current(&self) -> u8 {
        self.current
    }

    pub fn set(&mut self, new_value: u8) {
        if new_value > self.max {
            self.current = self.max;
        } else {
            self.current = new_value;
        }
    }

    pub fn set_max(&mut self, new_value: u8) {
        self.max = new_value;
        if self.current > self.max {
            self.current = self.max;
        }
    }
}

pub trait MutableHealth {
    fn health(&self) -> &Health;
    fn health_mut(&mut self) -> &mut Health;
}

pub trait IsDead: MutableHealth {
    fn is_dead(&self) -> bool {
        self.health().current < 1
    }
}

pub trait IsUnconscious: MutableHealth {
    fn is_unconscious(&self) -> bool {
        self.health().current < 6
    }
}
