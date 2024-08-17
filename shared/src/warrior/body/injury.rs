use serde::{Deserialize, Serialize};

use super::body_part::{BodySide, FingerName};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Injury {
    OneEyeGouged(BodySide),
    BothEyesGouged,
    KneeDislocated(BodySide),
    RightElbowDislocated,
    LeftElbowDislocated,
    RightShoulderDislocated,
    LeftShoulderDislocated,
    FootSmashed(BodySide),
    FootSevered(BodySide),
    OneLegBroken(BodySide),
    BothLegsBroken,
    OneLegSevered(BodySide),
    BothLegsSevered,
    RightArmBroken,
    RightArmSevered,
    RightHandBroken,
    RightHandSevered,
    LeftArmBroken,
    LeftArmSevered,
    LeftHandBroken,
    LeftHandSevered,
    GenitalsCrushed,
    FingerSevered(BodySide, FingerName),
}

impl PartialEq for Injury {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Injury::OneEyeGouged(side) => match other {
                Injury::OneEyeGouged(other_side) => other_side == side,
                _ => false,
            }
            Injury::BothEyesGouged => match other {
                Injury::BothEyesGouged => true,
                _ => false,
            },
            Injury::KneeDislocated(side) => match other {
                Injury::KneeDislocated(other_side) => other_side == side,
                _ => false,
            }
            Injury::RightElbowDislocated => match other {
                Injury::RightElbowDislocated => true,
                _ => false,
            },
            Injury::LeftElbowDislocated => match other {
                Injury::LeftElbowDislocated => true,
                _ => false,
            },
            Injury::RightShoulderDislocated => match other {
                Injury::RightShoulderDislocated => true,
                _ => false,
            },
            Injury::LeftShoulderDislocated => match other {
                Injury::LeftShoulderDislocated => true,
                _ => false,
            },
            Injury::FootSmashed(side) => match other {
                Injury::FootSmashed(other_side) => side == other_side,
                _ => false,
            },
            Injury::FootSevered(side) => match other {
                Injury::FootSevered(other_side) => side == other_side,
                _ => false,
            },
            Injury::OneLegBroken(side) => match other {
                Injury::OneLegBroken(other_side) => side == other_side,
                _ => false,
            },
            Injury::BothLegsBroken => match other {
                Injury::BothLegsBroken => true,
                _ => false,
            },
            Injury::OneLegSevered(side) => match other {
                Injury::OneLegSevered(other_side) => side == other_side,
                _ => false,
            },
            Injury::BothLegsSevered => match other {
                Injury::BothLegsSevered => true,
                _ => false,
            },
            Injury::RightArmBroken => match other {
                Injury::RightArmBroken => true,
                _ => false,
            },
            Injury::RightArmSevered => match other {
                Injury::RightArmSevered => true,
                _ => false,
            },
            Injury::RightHandBroken => match other {
                Injury::RightHandBroken => true,
                _ => false,
            },
            Injury::RightHandSevered => match other {
                Injury::RightHandSevered => true,
                _ => false,
            },
            Injury::LeftArmBroken => match other {
                Injury::LeftArmBroken => true,
                _ => false,
            },
            Injury::LeftArmSevered => match other {
                Injury::LeftArmSevered => true,
                _ => false,
            },
            Injury::LeftHandBroken => match other {
                Injury::LeftHandBroken => true,
                _ => false,
            },
            Injury::LeftHandSevered => match other {
                Injury::LeftHandSevered => true,
                _ => false,
            },
            Injury::GenitalsCrushed => match other {
                Injury::GenitalsCrushed => true,
                _ => false,
            },
            Injury::FingerSevered(side, finger) => match other {
                Injury::FingerSevered(other_side, other_finger) => side == other_side && finger == other_finger,
                _ => false,
            }
        }
    }
}

pub trait Injuries {
    fn injuries(&self) -> &Vec<Injury>;
    fn injuries_mut(&mut self) -> &mut Vec<Injury>;
    fn add_injury(&mut self, injury: Injury) {
        let mut i = 0;
        let len = self.injuries().len();
        while i < len {
            if self.injuries()[i] == injury {
                return;
            }
            i += 1;
        }
    }
}
