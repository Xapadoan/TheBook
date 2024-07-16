use std::error::Error;
use std::fmt::Display;

use crate::repository::main::Repository;
use crate::virtual_timer::VirtualTimer;
use crate::warrior::assault::damage_summary::ApplyDamageSummary;
use crate::warrior::{IsDead, IsUnconscious, Warrior};
use crate::warrior::weapon::MayHaveWeapon;
use crate::warrior::assault::Assault;

use super::replay::fight_replay::{FightReplayBuilder, FightReplayBuilderError};

#[derive(Debug)]
pub struct Fight {
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
            timer: VirtualTimer::new(),
        }
    }

    pub fn auto<T: Repository<Warrior>>(mut self, replay_builder: &mut FightReplayBuilder<T>) -> Result<FightResult, FightError> {
        replay_builder.record_warriors_init_state(&self.blue_corner, &self.red_corner)?;
        let mut turn: u8 = 0;

        while turn < u8::MAX {
            let blue_assault = self.blue_corner.assault(&mut self.red_corner);
            blue_assault.apply_damage_summary(
                &mut self.blue_corner,
                &mut self.red_corner,
            );
            replay_builder.push_assault(blue_assault);
            self.timer.add_time(2);
            let red_assault = self.red_corner.assault(&mut self.blue_corner);
            red_assault.apply_damage_summary(
                &mut self.red_corner,
                &mut self.blue_corner,
            );
            replay_builder.push_assault(red_assault);
            self.timer.add_time(2);
            turn += 1;
            self.blue_corner.apply_duration_damages(self.timer.absolute_time());
            self.red_corner.apply_duration_damages(self.timer.absolute_time());
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
