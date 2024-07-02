// pub mod show_action;

use crate::virtual_timer::VirtualTimer;
use crate::warrior::assault::damage_summary::ApplyDamageSummary;
use crate::warrior::{IsDead, IsUnconscious, Warrior};
use crate::warrior::weapon::MayHaveWeapon;
use crate::warrior::assault::Assault;
use crate::name::HasName;

#[derive(Debug)]
pub struct Fight {
    blue_corner: Warrior,
    red_corner: Warrior,
    timer: VirtualTimer,
}

pub struct FightResult {
    winner: Option<Warrior>,
    end_reason: String,
}

impl FightResult {
    pub fn winner(self) -> Option<Warrior> {
        return self.winner;
    }

    pub fn end_reason(&self) -> &String {
        return &self.end_reason;
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

    pub fn fighters(&self) -> (&String, &String) {
        (self.blue_corner.name(), self.red_corner.name())
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

    pub fn auto(mut self) -> FightResult {
        let mut turn: u8 = 0;

        self.blue_corner.present_self();
        self.red_corner.present_self();

        while turn < u8::MAX {
            println!("=== {turn} ===");
            let blue_assault = self.blue_corner.assault(&mut self.red_corner);
            dbg!(&blue_assault);
            blue_assault.apply_damage_summary(&mut self.blue_corner, &mut self.red_corner);
            self.timer.add_time(2);
            let red_assault = self.red_corner.assault(&mut self.blue_corner);
            dbg!(&red_assault);
            red_assault.apply_damage_summary(&mut self.red_corner, &mut self.blue_corner);
            self.timer.add_time(2);
            println!("\n");
            turn += 1;
            self.blue_corner.apply_duration_damages(self.timer.absolute_time());
            self.red_corner.apply_duration_damages(self.timer.absolute_time());
            if self.blue_corner.is_dead() || self.blue_corner.is_unconscious() || self.blue_corner.weapon().is_none() {
                return FightResult {
                    winner: Some(self.red_corner),
                    end_reason: Fight::end_reason(&self.blue_corner),
                };
            }
            if self.red_corner.is_dead() || self.red_corner.is_unconscious() || self.red_corner.weapon().is_none() {
                return FightResult {
                    winner: Some(self.blue_corner),
                    end_reason: Fight::end_reason(&self.red_corner),
                };
            }
        }

        return FightResult {
            winner: None,
            end_reason: String::from("public got bored and left"),
        };
    }
}
