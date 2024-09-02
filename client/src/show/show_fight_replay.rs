use shared::inventory::{Inventory, Items};
use shared::replay::FightReplay;
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;
use uuid::Uuid;

use crate::show::ShowSelf;

use super::show_turn_summary::ShowTurnSummary;

pub trait ShowFightReplay {
    fn show_fight_replay(&self, warriors: (&mut Warrior, &mut Warrior));
}

impl ShowFightReplay for FightReplay {
    fn show_fight_replay(&self, warriors: (&mut Warrior, &mut Warrior)) {
        let (blue_corner, red_corner) = get_corners(self, warriors);
        let mut blue_corner_dropped_items = Inventory::new();
        let mut red_corner_dropped_items = Inventory::new();
        for turn in self.turn_summaries() {
            println!("=== BEGIN TURN ===");
            println!("{}", turn.show_turn_summary(
                blue_corner,
                &mut blue_corner_dropped_items,
                red_corner,
                &mut red_corner_dropped_items,
            ));
            println!("==== END TURN ====\n");
        }
    }
}

pub trait ShowWarriorFightReplay {
    fn show_warrior_fight_replay(&self, warriors: (&mut Warrior, &mut Warrior), warrior_uuid: &Uuid);
}

impl ShowWarriorFightReplay for FightReplay {
    fn show_warrior_fight_replay(&self, warriors: (&mut Warrior, &mut Warrior), warrior_uuid: &Uuid) {
        let (blue_corner, red_corner) = get_corners(self, warriors);
        let mut blue_corner_dropped_items = Inventory::new();
        let mut red_corner_dropped_items = Inventory::new();
        for turn in self.turn_summaries() {
            println!("=== BEGIN TURN ===");
            println!("{}", turn.show_turn_summary(
                blue_corner,
                &mut blue_corner_dropped_items,
                red_corner,
                &mut red_corner_dropped_items,
            ));
            println!("==== END TURN ====\n");
        }
        if blue_corner.uuid() == warrior_uuid {
            show_dropped_items(blue_corner, &blue_corner_dropped_items);
        } else {
            show_dropped_items(red_corner, &red_corner_dropped_items);
        }
    }
}

fn get_corners<'a>(replay: &FightReplay, warriors: (&'a mut Warrior, &'a mut Warrior)) -> (&'a mut Warrior, &'a mut Warrior) {
    if replay.blue_corner_uuid() == warriors.0.uuid() {
        (warriors.0, warriors.1)
    } else {
        (warriors.1, warriors.0)
    }
}

fn show_dropped_items(warrior: &Warrior, dropped_items: &Inventory) {
    if dropped_items.items().len() < 1 { return }
    println!("During the fight, {} lost:", warrior.show_self());
    for item in dropped_items.items() {
        println!("\t{}", item.show_self())
    }
    println!("Those are not lost, you can find them in your inventory");
}
