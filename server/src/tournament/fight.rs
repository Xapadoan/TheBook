use std::error::Error;
use std::fmt::Display;

use shared::equipment::weapon::OptionalMutableWeapon;
use shared::experience::GainExperience;
use shared::health::{IsDead, IsUnconscious};
use shared::inventory::Inventory;
use shared::replay::turn_summary::TurnSummary;
use shared::replay::FightReplaySummary;
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;

use crate::repository::Repository;
use crate::replay::{FightReplayBuilder, FightReplayBuilderError};

#[derive(Debug)]
pub struct Fight {}

impl Fight {
    pub fn auto<T: Repository<Warrior>>(
        replay_builder: &mut FightReplayBuilder<T>,
        blue_corner: &mut Warrior,
        blue_corner_dropped_items: &mut Inventory,
        red_corner: &mut Warrior,
        red_corner_dropped_items: &mut Inventory,
    ) -> Result<FightReplaySummary, FightError> {
        replay_builder.record_warriors_init_state(blue_corner, red_corner)?;
        let mut turn: u8 = 0;

        while turn < u8::MAX {
            let turn_summary = TurnSummary::new(
                blue_corner,
                blue_corner_dropped_items,
                red_corner,
                red_corner_dropped_items,
            );
            replay_builder.push_turn_summary(turn_summary);
            turn += 1;
            if blue_corner.is_dead()
                || blue_corner.is_unconscious()
                || blue_corner.weapon().is_none()
            {
                red_corner.gain_xp(20);
                let result = FightReplaySummary::new(
                    replay_builder.replay_uuid().clone(),
                    Some(red_corner.uuid().clone()),
                    blue_corner.uuid().clone(),
                    red_corner.uuid().clone(),
                );
                return Ok(result);
            }
            replay_builder.replay_uuid();
            if red_corner.is_dead()
                || red_corner.is_unconscious()
                || red_corner.weapon().is_none()
            {
                blue_corner.gain_xp(20);
                let result = FightReplaySummary::new(
                    replay_builder.replay_uuid().clone(),
                    Some(blue_corner.uuid().clone()),
                    blue_corner.uuid().clone(),
                    red_corner.uuid().clone(),
                );
                return Ok(result);
            }
        }

        let result = FightReplaySummary::new(
                    replay_builder.replay_uuid().clone(),
                    None,
                    blue_corner.uuid().clone(),
                    red_corner.uuid().clone(),
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
