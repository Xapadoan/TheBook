use std::error::Error;
use std::fmt::Display;

use shared::assault::assault_order_comparable::AssaultOrderComparable;
use shared::equipment::weapon::OptionalMutableWeapon;
use shared::health::{IsDead, IsUnconscious};
use shared::replay::turn_summary::TurnSummary;
use shared::replay::FightReplaySummary;
use shared::tournament::Fighter;
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;

use crate::repository::Repository;
use crate::replay::{FightReplayBuilder, FightReplayBuilderError};

#[derive(Debug)]
pub struct Fight {}

impl Fight {
    pub fn auto<T: Repository<Warrior>>(
        replay_builder: &mut FightReplayBuilder<T>,
        blue_corner: &mut Fighter,
        red_corner: &mut Fighter,
    ) -> Result<FightReplaySummary, FightError> {
        let mut turn: u8 = 0;
        let blue_corner_uuid = blue_corner.uuid().clone();
        let red_corner_uuid = red_corner.uuid().clone();

        let (first_assailant, second_assailant) = if blue_corner.assault_order_comparable() > red_corner.assault_order_comparable() {
            (blue_corner, red_corner)
        } else {
            (red_corner, blue_corner)
        };

        while turn < u8::MAX {
            let turn_summary = TurnSummary::new(
                first_assailant,
                second_assailant,
            );
            replay_builder.push_turn_summary(turn_summary);
            turn += 1;
            if first_assailant.is_dead()
                || first_assailant.is_unconscious()
                || first_assailant.weapon().is_none()
            {
                let result = FightReplaySummary::new(
                    replay_builder.replay_uuid().clone(),
                    Some(second_assailant.uuid().clone()),
                    blue_corner_uuid,
                    red_corner_uuid,
                );
                return Ok(result);
            }
            replay_builder.replay_uuid();
            if second_assailant.is_dead()
                || second_assailant.is_unconscious()
                || second_assailant.weapon().is_none()
            {
                let result = FightReplaySummary::new(
                    replay_builder.replay_uuid().clone(),
                    Some(first_assailant.uuid().clone()),
                    blue_corner_uuid,
                    red_corner_uuid,
                );
                return Ok(result);
            }
        }

        let result = FightReplaySummary::new(
            replay_builder.replay_uuid().clone(),
            None,
            blue_corner_uuid,
            red_corner_uuid,
        );
        return Ok(result);
    }
}

#[derive(Debug)]
pub struct FightError {
    message: String,
}

impl FightError {
    fn new(message: String) -> Self {
        Self { message: format!("Fight Error:\n{message}") }
    }
}

impl Display for FightError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for FightError {}

impl From<FightReplayBuilderError> for FightError {
    fn from(value: FightReplayBuilderError) -> Self {
        Self::new(format!("Fight Builder Error: \n{value}"))
    }
}
