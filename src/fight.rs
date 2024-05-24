use crate::warrior::Warrior;

#[derive(Debug)]
pub struct Fight {
    blue_corner: Warrior,
    red_corner: Warrior,
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
    pub fn new(blue_corner: Warrior, red_corner: Warrior) -> Fight {
        Fight {
            blue_corner,
            red_corner,
        }
    }

    pub fn fighters(&self) -> (&String, &String) {
        (self.blue_corner.name(), self.red_corner.name())
    }

    pub fn auto(mut self) -> FightResult {
        let mut turn: u8 = 0;

        self.blue_corner.present_self();
        self.red_corner.present_self();

        while self.blue_corner.is_alive() && self.red_corner.is_alive() && turn < 255 {
            println!("=== {turn} ===");
            self.blue_corner.attack(&mut self.red_corner);
            self.red_corner.attack(&mut self.blue_corner);
            println!("\n");
            turn += 1;
        }

        if turn < 255 {
            if self.blue_corner.is_alive() {
                let winner_name = self.blue_corner.name().clone();
                return FightResult {
                    winner: Some(self.blue_corner),
                    end_reason: format!("{} winned", winner_name),
                };
            }
            if self.red_corner.is_alive() {
                let winner_name = self.red_corner.name().clone();
                return FightResult {
                    winner: Some(self.red_corner),
                    end_reason: format!("{} winned", winner_name),
                };
            }
            return FightResult {
                winner: None,
                end_reason: String::from("both died"),
            };
        }
        return FightResult {
            winner: None,
            end_reason: String::from("public got bored and left"),
        };
    }
}
