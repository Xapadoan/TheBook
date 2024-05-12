use rand::Rng;

struct Warrior {
    name: String,
    health: u8,
    attack: u8,
    parry: u8,
}

impl Warrior {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            health: 30,
            attack: 8,
            parry: 10,
        }
    }

    fn present_self(&self) {
        println!("Hi ! I'm {}", self.name);
    }

    // Fast exit make code more readable ?
    fn attack(&self, target: &mut Warrior) {
        println!("{} attacks {}", self.name, target.name);
        match self.roll_attack() {
            RollResult::Failure => println!("{} missed", self.name),
            RollResult::Success => match target.roll_parry() {
                RollResult::Success => println!("{} parried", target.name),
                RollResult::Failure => {
                    let dmg_score = Dice::D6.roll();
                    if dmg_score > 2 {
                        target.take_damage(dmg_score - 2);
                    }
                    println!("{} was hit", target.name)
                }
            },
        }
    }

    fn roll_attack(&self) -> RollResult {
        if Dice::D20.roll() > self.attack {
            return RollResult::Failure;
        }
        RollResult::Success
    }

    fn roll_parry(&self) -> RollResult {
        if Dice::D20.roll() > self.parry {
            return RollResult::Failure;
        }
        RollResult::Success
    }

    fn take_damage(&mut self, dmg: u8) {
        if self.health > dmg {
            self.health -= dmg;
        } else {
            self.health = 0;
        }
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

enum Dice {
    D20,
    D6,
}

impl Dice {
    fn roll(&self) -> u8 {
        let rng = match self {
            Dice::D20 => 1..20,
            Dice::D6 => 1..6,
        };
        rand::thread_rng().gen_range(rng)
    }
}

enum RollResult {
    Success,
    Failure,
}

fn main() {
    let mut turn: u8 = 0;
    let mut masarma = Warrior::new("Masarma");
    let mut lehtobel = Warrior::new("Lehtobel");

    masarma.present_self();
    lehtobel.present_self();

    while masarma.is_alive() && lehtobel.is_alive() && turn < 255 {
        println!("=== {turn} ===");
        masarma.attack(&mut lehtobel);
        lehtobel.attack(&mut masarma);
        println!("\n");
        turn += 1;
    }

    if !masarma.is_alive() {
        println!("{} wins !!", lehtobel.name);
    } else if !lehtobel.is_alive() {
        println!("{} wins !!", masarma.name);
    } else {
        println!("Public got bored and left");
    }
}
