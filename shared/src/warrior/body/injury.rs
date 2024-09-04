use serde::{Deserialize, Serialize};

use crate::stats::StatModifier;

use super::body_part::{BodyPart, BodySide, FingerName, OptionalMutableBodyPart};

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

pub trait Injuries: OptionalMutableBodyPart {
    fn injuries(&self) -> Vec<Injury>;
    fn add_injury(&mut self, injury: Injury) -> Vec<BodyPart>;
}

impl StatModifier for Injury {
    fn attack_mod(&self) -> i8 {
        match self {
            Self::BothEyesGouged => -5,
            Self::BothLegsBroken |
            Self::BothLegsSevered => -8,
            Self::FingerSevered(_, _) => 0,
            Self::FootSevered(_) |
            Self::FootSmashed(_) => -2,
            Self::GenitalsCrushed => 0,
            Self::KneeDislocated(_) => -1,
            Self::LeftArmBroken => -2,
            Self::LeftArmSevered => -3,
            Self::LeftElbowDislocated |
            Self::LeftShoulderDislocated => -1,
            Self::LeftHandBroken |
            Self::LeftHandSevered => -2,
            Self::OneEyeGouged(_) => -1,
            Self::OneLegBroken(_) |
            Self::OneLegSevered(_) => -4,
            Self::RightArmBroken |
            Self::RightArmSevered => -5,
            Self::RightElbowDislocated |
            Self::RightShoulderDislocated => -1,
            Self::RightHandBroken |
            Self::RightHandSevered => -5,
        }
    }
    fn parry_mod(&self) -> i8 {
        match self {
            Self::BothEyesGouged => -8,
            Self::BothLegsBroken |
            Self::BothLegsSevered => -8,
            Self::FingerSevered(_, _) => 0,
            Self::FootSevered(_) |
            Self::FootSmashed(_) => -2,
            Self::GenitalsCrushed => 0,
            Self::KneeDislocated(_) => -2,
            Self::LeftArmBroken => -3,
            Self::LeftArmSevered => -4,
            Self::LeftElbowDislocated |
            Self::LeftShoulderDislocated => -2,
            Self::LeftHandBroken |
            Self::LeftHandSevered => -3,
            Self::OneEyeGouged(_) => -2,
            Self::OneLegBroken(_) |
            Self::OneLegSevered(_) => -6,
            Self::RightArmBroken |
            Self::RightArmSevered => -6,
            Self::RightElbowDislocated |
            Self::RightShoulderDislocated => -2,
            Self::RightHandBroken |
            Self::RightHandSevered => -6,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stats::Stat;

    use super::*;

    #[test]
    fn foot_severed_stats_modifier() {
        let attack = Stat::Attack(10);
        let parry = Stat::Parry(10);
        let injury = Injury::FootSevered(BodySide::Left);

        assert_eq!(injury.modify_stat(attack).value(), 8);
        assert_eq!(injury.modify_stat(parry).value(), 8);
    }
}