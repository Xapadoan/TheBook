use the_book::tournament::Tournament;
use the_book::warrior::Warrior;
use the_book::weapon::Weapon;
use the_book::weapon::WeaponKind;

fn main() {
    let mut tournament = Tournament::new(vec![
        Warrior::new("Masarma", Weapon::new(WeaponKind::GreatSword)),
        // Warrior::new("Masarma", Weapon::GreatSword(GreatSword::new())),
        Warrior::new("Lehtobel", Weapon::new(WeaponKind::Sword)),
        // Warrior::new("Lehtobel", Weapon::Sword(Sword::new())),
        Warrior::new("Tendark", Weapon::new(WeaponKind::BattleAxe)),
        // Warrior::new("Tendark", Weapon::BattleAxe(BattleAxe::new())),
        Warrior::new("Arcen", Weapon::new(WeaponKind::Sword)),
        // Warrior::new("Arcen", Weapon::Sword(Sword::new())),
        Warrior::new("Morbiff", Weapon::new(WeaponKind::Axe)),
        // Warrior::new("Morbiff", Weapon::Axe(Axe::new())),
        Warrior::new("Nithu", Weapon::new(WeaponKind::WarHammer)),
        // Warrior::new("Nithu", Weapon::WarHammer(WarHammer::new())),
        Warrior::new("Finul", Weapon::new(WeaponKind::Hammer)),
        // Warrior::new("Finul", Weapon::Hammer(Hammer::new())),
        Warrior::new("Chei", Weapon::new(WeaponKind::Hammer)),
    ]);

    tournament.fight_round(0);
}
