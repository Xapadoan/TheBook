use std::error::Error;
use std::fmt::Display;

use shared::assault::assault_summary::AssaultSummary;
use shared::end_turn_consequences::EndTurnConsequencesBuilder;
use shared::equipment::weapon::OptionalMutableWeapon;
use shared::health::{IsDead, IsUnconscious};
use shared::warrior::Warrior;

use crate::repository::main::Repository;

use super::replay::fight_replay::{FightReplayBuilder, FightReplayBuilderError};

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
}

impl FightResult {
    pub fn kind(&self) -> &FightResultKind {
        &self.kind
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
            let blue_assault = AssaultSummary::new(&self.blue_corner, &self.red_corner);
            blue_assault.consequences().apply(
                &mut self.blue_corner,
                &mut self.red_corner,
            );
            replay_builder.push_assault(blue_assault);
            let red_assault = AssaultSummary::new(&self.red_corner, &self.blue_corner);
            red_assault.consequences().apply(
                &mut self.red_corner,
                &mut self.blue_corner,
            );
            replay_builder.push_assault(red_assault);
            turn += 1;
            self.blue_corner.end_turn();
            self.red_corner.end_turn();
            if self.blue_corner.is_dead()
                || self.blue_corner.is_unconscious()
                || self.blue_corner.weapon().is_none()
            {
                let result = FightResult {
                    kind: FightResultKind::Victory(
                        Fighters { winner: self.red_corner, loser: self.blue_corner }
                    ),
                };
                return Ok(result);
            }
            if self.red_corner.is_dead()
                || self.red_corner.is_unconscious()
                || self.red_corner.weapon().is_none()
            {
                let result = FightResult {
                    kind: FightResultKind::Victory(
                        Fighters { winner: self.blue_corner, loser: self.red_corner }
                    ),
                };
                return Ok(result);
            }
        }

        let result = FightResult {
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
