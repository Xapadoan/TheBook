use std::fmt::Display;

use crate::dice::Dice;
use crate::gen_random::GenRandom;

#[derive(Debug)]
#[derive(Clone)]
pub enum BodySide {
    Left,
    Right,
}

impl GenRandom for  BodySide {
    fn gen_random() -> Self {
        match Dice::D6.roll() {
            1..=3 => Self::Right,
            4..=6 => Self::Left,
            other => panic!("D6 roll resulted in {other}")
        }
    }
}

impl Display for BodySide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodySide::Left => write!(f, "left"),
            BodySide::Right => write!(f, "right"),
        }
    }
}