use shared::inventory::HasInventory;
use shared::replay::FightReplay;
use shared::tournament::Fighter;
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;
use uuid::Uuid;

use crate::show::ShowSelf;

use super::show_turn_summary::ShowTurnSummary;

pub trait ShowWarriorFightReplay {
    fn show_warrior_fight_replay(&self, warriors: (&mut Warrior, &mut Warrior), warrior_uuid: &Uuid);
}

impl ShowWarriorFightReplay for FightReplay {
    fn show_warrior_fight_replay(&self, warriors: (&mut Warrior, &mut Warrior), warrior_uuid: &Uuid) {
        let (blue_corner_warrior, red_corner_warrior) = get_corners(self, warriors);
        let mut blue_corner = Fighter::from(&*blue_corner_warrior);
        let mut red_corner = Fighter::from(&*red_corner_warrior);
        for turn in self.turn_summaries() {
            println!("=== BEGIN TURN ===");
            println!("{}", turn.show_turn_summary(
                &mut blue_corner,
                &mut red_corner,
            ));
            println!("==== END TURN ====\n");
        }
        if blue_corner.uuid() == warrior_uuid {
            show_dropped_items(&blue_corner);
        } else {
            show_dropped_items(&red_corner);
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

fn show_dropped_items(fighter: &Fighter) {
    if fighter.inventory().items().len() < 1 { return }
    println!("During the fight, {} lost:", fighter.show_self());
    for (_, item) in fighter.inventory().items() {
        println!("\t{}", item.show_self())
    }
    println!("Those are not lost, you can find them in your inventory");
}
