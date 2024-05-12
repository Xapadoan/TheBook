use crate::warrior::Warrior;

pub struct Arena {
    name: String,
    blue_corner: Warrior,
    red_corner: Warrior,
}

impl Arena {
    pub fn new(name: &str, blue_corner: Warrior, red_corner: Warrior) -> Arena {
        Arena {
            name: String::from(name),
            blue_corner,
            red_corner,
        }
    }

    pub fn fight(&mut self) {
        let mut turn: u8 = 0;

        println!("A fight is about to start in {}", self.name);

        self.blue_corner.present_self();
        self.red_corner.present_self();

        while self.blue_corner.is_alive() && self.red_corner.is_alive() && turn < 255 {
            println!("=== {turn} ===");
            self.blue_corner.attack(&mut self.red_corner);
            self.red_corner.attack(&mut self.blue_corner);
            dbg!(&self.blue_corner);
            dbg!(&self.red_corner);
            println!("\n");
            turn += 1;
        }

        if turn < 255 {
            if self.blue_corner.is_alive() {
                println!("{} wins !!", self.blue_corner.name());
            } else if self.red_corner.is_alive() {
                println!("{} wins !!", self.red_corner.name());
            } else {
                println!("Oops... Both died.")
            }
        } else {
            println!("Public got bored and left")
        }
    }
}
