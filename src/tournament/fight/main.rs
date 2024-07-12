use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repository::main::Repository;
use crate::virtual_timer::VirtualTimer;
use crate::warrior::assault::damage_summary::ApplyDamageSummary;
use crate::warrior::{IsDead, IsUnconscious, Warrior};
use crate::warrior::weapon::MayHaveWeapon;
use crate::warrior::assault::Assault;
use crate::name::HasName;

use super::replay_data::{FightReplayBuilder, FightReplayBuilderError};

#[derive(Debug)]
pub struct Fight {
    tournament_uuid: Uuid,
    round_index: u8,
    blue_corner: Warrior,
    red_corner: Warrior,
    timer: VirtualTimer,
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
    end_reason: String,
}

impl FightResult {
    pub fn kind(&self) -> &FightResultKind {
        &self.kind
    }

    pub fn end_reason(&self) -> &str {
        &self.end_reason
    }
}

impl Fight {
    pub fn new(tournament_uuid: Uuid, round_index: u8, blue_corner: Warrior, red_corner: Warrior) -> Self {
        // println!("{} will fight {}", blue_corner.name(), red_corner.name());
        Self {
            tournament_uuid,
            round_index,
            blue_corner,
            red_corner,
            timer: VirtualTimer::new(),
        }
    }

    fn end_reason(loser: &Warrior) -> String {
        if loser.is_dead() {
            format!("{} is dead", loser.name())
        } else if loser.is_unconscious() {
            format!("{} is unconscious", loser.name())
        } else {
            format!("{} has no weapon", loser.name())
        }
    }

    pub fn auto<T: Repository<Warrior>>(mut self, replay_builder: &mut FightReplayBuilder<T>) -> Result<FightResult, FightError> {
        // let mut replay_builder = FightReplayBuilder::build(self.tournament_uuid, self.round_index)?;
        replay_builder.record_warriors_init_state(&self.blue_corner, &self.red_corner)?;
        let mut turn: u8 = 0;

        // self.blue_corner.present_self();
        // self.red_corner.present_self();

        while turn < u8::MAX {
            // println!("=== {turn} ===");
            let blue_assault = self.blue_corner.assault(&mut self.red_corner);
            // dbg!(&blue_assault);
            blue_assault.apply_damage_summary(
                &mut self.blue_corner,
                &mut self.red_corner,
            );
            replay_builder.push_assault(blue_assault);
            self.timer.add_time(2);
            let red_assault = self.red_corner.assault(&mut self.blue_corner);
            // dbg!(&red_assault);
            red_assault.apply_damage_summary(
                &mut self.red_corner,
                &mut self.blue_corner,
            );
            replay_builder.push_assault(red_assault);
            self.timer.add_time(2);
            // println!("\n");
            turn += 1;
            self.blue_corner.apply_duration_damages(self.timer.absolute_time());
            self.red_corner.apply_duration_damages(self.timer.absolute_time());
            if self.blue_corner.is_dead()
                || self.blue_corner.is_unconscious()
                || self.blue_corner.weapon().is_none()
            {
                let end_reason = Self::end_reason(&self.blue_corner);
                // println!("{} was eliminated because {}", self.blue_corner.name(), end_reason);
                let result = FightResult {
                    kind: FightResultKind::Victory(
                        Fighters { winner: self.red_corner, loser: self.blue_corner }
                    ),
                    end_reason,
                };
                // replay_builder.write_summary(&result)?;
                return Ok(result);
            }
            if self.red_corner.is_dead()
                || self.red_corner.is_unconscious()
                || self.red_corner.weapon().is_none()
            {
                let end_reason = Self::end_reason(&self.red_corner);
                // println!("{} was eliminated because {}", self.red_corner.name(), end_reason);
                let result = FightResult {
                    kind: FightResultKind::Victory(
                        Fighters { winner: self.blue_corner, loser: self.red_corner }
                    ),
                    end_reason,
                };
                // replay_builder.write_summary(&result)?;
                return Ok(result);
            }
        }

        let result = FightResult {
            kind: FightResultKind::Tie((self.blue_corner, self.red_corner)),
            end_reason: String::from("public got bored and left"),
        };
        // replay_builder.write_summary(&result)?;
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
