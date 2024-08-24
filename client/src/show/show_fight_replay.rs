use shared::replay::FightReplay;
use shared::unique_entity::UniqueEntity;
use shared::warrior::Warrior;

use super::show_turn_summary::ShowTurnSummary;

pub trait ShowFightReplay {
    fn show_fight_replay(&self, warriors: (&mut Warrior, &mut Warrior));
}

impl ShowFightReplay for FightReplay {
    fn show_fight_replay(&self, warriors: (&mut Warrior, &mut Warrior)) {
        let (blue_corner, red_corner) = get_corners(self, warriors);
        for turn in self.turn_summaries() {
            println!("=== BEGIN TURN ===");
            println!("{}", turn.show_turn_summary(blue_corner, red_corner));
            println!("==== END TURN ====\n");
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
