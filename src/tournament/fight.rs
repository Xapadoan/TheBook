// pub mod show_action;

use crate::virtual_timer::VirtualTimer;
use crate::warrior::assault::damage_summary::ApplyDamageSummary;
use crate::warrior::{IsDead, IsUnconscious, Warrior};
use crate::warrior::weapon::MayHaveWeapon;
use crate::warrior::assault::Assault;
use crate::name::HasName;

#[derive(Debug)]
pub struct Fight<'f> {
    blue_corner: &'f mut Warrior,
    red_corner: &'f mut Warrior,
    timer: VirtualTimer,
}

pub struct FightResult<'fr> {
    winner: Option<&'fr mut Warrior>,
    loser: Option<&'fr mut Warrior>,
    end_reason: String,
}

impl<'fr> FightResult<'fr> {
    pub fn winner(&mut self) -> Option<&'fr mut Warrior> {
        return self.winner.take();
    }

    pub fn loser(&mut self) -> Option<&'fr mut Warrior> {
        return self.loser.take();
    }

    pub fn end_reason(&self) -> &String {
        return &self.end_reason;
    }
}

impl<'f> Fight<'f> {
    pub fn new(blue_corner: &'f mut Warrior, red_corner: &'f mut Warrior) -> Self {
        println!("{} will fight {}", blue_corner.name(), red_corner.name());
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

    pub fn auto<'a>(&'a mut self) -> FightResult {
        let mut turn: u8 = 0;

        self.blue_corner.present_self();
        self.red_corner.present_self();

        while turn < u8::MAX {
            println!("=== {turn} ===");
            let blue_assault = self.blue_corner.assault(self.red_corner);
            dbg!(&blue_assault);
            blue_assault.apply_damage_summary(self.blue_corner, self.red_corner);
            self.timer.add_time(2);
            let red_assault = self.red_corner.assault(self.blue_corner);
            dbg!(&red_assault);
            red_assault.apply_damage_summary(self.red_corner, self.blue_corner);
            self.timer.add_time(2);
            println!("\n");
            turn += 1;
            self.blue_corner.apply_duration_damages(self.timer.absolute_time());
            self.red_corner.apply_duration_damages(self.timer.absolute_time());
            if self.blue_corner.is_dead() || self.blue_corner.is_unconscious() || self.blue_corner.weapon().is_none() {
                let end_reason = Fight::end_reason(self.blue_corner);
                return FightResult {
                    winner: Some(self.red_corner),
                    loser: Some(self.blue_corner),
                    end_reason,
                };
            }
            if self.red_corner.is_dead() || self.red_corner.is_unconscious() || self.red_corner.weapon().is_none() {
                let end_reason = Fight::end_reason(self.red_corner);
                return FightResult {
                    winner: Some(self.blue_corner),
                    loser: Some(self.red_corner),
                    end_reason,
                };
            }
        }

        return FightResult {
            winner: None,
            loser: None,
            end_reason: String::from("public got bored and left"),
        };
    }
}
