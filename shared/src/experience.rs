use std::{error::Error, fmt::Display};

use crate::stats::Stat;

pub trait Experience {
    fn xp(&self) -> u64;
    fn level(&self) -> u8;
}

pub trait GainExperience: Experience {
    fn gain_xp(&mut self, xp: u64);
    fn can_level_up(&self) -> bool {
        self.xp() > level_xp_threshold(self.level() + 1)
    }
    fn level_up(&mut self, stat: &Stat) -> Result<(), ExperienceError>;
}

fn level_xp_threshold(level_to_reach: u8) -> u64 {
    let mut xp: u64 = 0;
    let mut lvl = 1;
    let mut level_gap = 100;

    while lvl < level_to_reach {
        xp += level_gap;
        if lvl < 20 {
            level_gap += 100;
        } else {
            level_gap += 1000;
        }
        lvl += 1;
    }

    xp
}

#[derive(Debug)]
pub enum ExperienceErrorKind {
    InvalidStatIncrement(u8, Stat),
}

#[derive(Debug)]
pub struct ExperienceError {
    message: String,
}

impl ExperienceError {
    pub fn new(kind: &ExperienceErrorKind) -> Self {
        match &kind {
            &ExperienceErrorKind::InvalidStatIncrement(lvl, stat) => Self {
                message: format!("Invalid Stat Increment ({:?}, {:?})", lvl, stat),
            },
        }
    }
}

impl Display for ExperienceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ExperienceError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_xp_threshold_coherence() {
        let expected = [
            (1, 0),
            (2, 100),
            (3, 300),
            (4, 600),
            (5, 1000),
            (10, 4500),
            (15, 10500),
            (20, 19000),
            (21, 21000),
            (22, 24000),
            (23, 28000),
            (24, 33000),
            (25, 39000),
        ];
        for (level, expected_xp) in expected {
            assert_eq!(expected_xp, level_xp_threshold(level))
        }
    }
}
