use the_book::tournament::Tournament;
use the_book::warrior::Warrior;
use the_book::weapon::axe::Axe;
use the_book::weapon::battle_axe::BattleAxe;
use the_book::weapon::great_sword::GreatSword;
use the_book::weapon::hammer::Hammer;
use the_book::weapon::sword::Sword;
use the_book::weapon::war_hammer::WarHammer;
use the_book::weapon::Weapon;

fn main() {
    let mut tournament = Tournament::new(vec![
        Warrior::new("Masarma", Weapon::GreatSword(GreatSword::new())),
        Warrior::new("Lehtobel", Weapon::Sword(Sword::new())),
        Warrior::new("Tendark", Weapon::BattleAxe(BattleAxe::new())),
        Warrior::new("Arcen", Weapon::Sword(Sword::new())),
        Warrior::new("Morbiff", Weapon::Axe(Axe::new())),
        Warrior::new("Nithu", Weapon::WarHammer(WarHammer::new())),
        Warrior::new("Finul", Weapon::Hammer(Hammer::new())),
        Warrior::new("Chei", Weapon::Hammer(Hammer::new())),
    ]);

    tournament.fight_round(0);
}
