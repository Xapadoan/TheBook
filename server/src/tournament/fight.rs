use std::error::Error;
use std::fmt::Display;

use shared::equipment::weapon::OptionalMutableWeapon;
use shared::experience::GainExperience;
use shared::health::{IsDead, IsUnconscious};
use shared::replay::turn_summary::TurnSummary;
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;
use uuid::Uuid;

use crate::repository::Repository;
use crate::replay::{FightReplayBuilder, FightReplayBuilderError};

#[derive(Debug)]
pub struct Fight {
    blue_corner: Warrior,
    red_corner: Warrior,
}

#[derive(Debug)]
pub struct Fighters {
    winner: Warrior,
    loser: Warrior,
}

impl Fighters {
    pub fn winner(&self) -> &Warrior {
        &self.winner
    }

    pub fn loser(&self) -> &Warrior {
        &self.loser
    }
}

#[derive(Debug)]
pub enum FightResultKind {
    Tie((Warrior, Warrior)),
    Victory(Fighters)
}

#[derive(Debug)]
pub struct FightResult {
    kind: FightResultKind,
    blue_corner_uuid: Uuid,
    red_corner_uuid: Uuid,
}

impl FightResult {
    pub fn kind(&self) -> &FightResultKind {
        &self.kind
    }
    pub fn blue_corner_uuid(&self) -> &Uuid {
        &self.blue_corner_uuid
    }
    pub fn red_corner_uuid(&self) -> &Uuid {
        &self.red_corner_uuid
    }
}

impl Fight {
    pub fn new(blue_corner: Warrior, red_corner: Warrior) -> Self {
        Self {
            blue_corner,
            red_corner,
        }
    }

    pub fn auto<T: Repository<Warrior>>(mut self, replay_builder: &mut FightReplayBuilder<T>) -> Result<FightResult, FightError> {
        replay_builder.record_warriors_init_state(&self.blue_corner, &self.red_corner)?;
        let mut turn: u8 = 0;

        while turn < u8::MAX {
            let turn_summary = TurnSummary::new(&mut self.blue_corner, &mut self.red_corner);
            replay_builder.push_turn_summary(turn_summary);
            turn += 1;
            if self.blue_corner.is_dead()
                || self.blue_corner.is_unconscious()
                || self.blue_corner.weapon().is_none()
            {
                self.red_corner.gain_xp(20);
                let result = FightResult {
                    blue_corner_uuid: self.blue_corner.uuid().clone(),
                    red_corner_uuid: self.red_corner.uuid().clone(),
                    kind: FightResultKind::Victory(
                        Fighters { winner: self.red_corner, loser: self.blue_corner },
                    ),
                };
                return Ok(result);
            }
            if self.red_corner.is_dead()
                || self.red_corner.is_unconscious()
                || self.red_corner.weapon().is_none()
            {
                self.blue_corner.gain_xp(20);
                let result = FightResult {
                    blue_corner_uuid: self.blue_corner.uuid().clone(),
                    red_corner_uuid: self.red_corner.uuid().clone(),
                    kind: FightResultKind::Victory(
                        Fighters { winner: self.blue_corner, loser: self.red_corner }
                    ),
                };
                return Ok(result);
            }
        }

        let result = FightResult {
            blue_corner_uuid: self.blue_corner.uuid().clone(),
            red_corner_uuid: self.red_corner.uuid().clone(),
            kind: FightResultKind::Tie((self.blue_corner, self.red_corner)),
        };
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
