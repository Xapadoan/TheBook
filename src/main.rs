use the_book::tournament::Tournament;
use the_book::warrior::Warrior;
use the_book::weapon::Weapon;
use the_book::weapon::WeaponKind;

fn main() {
    let mut tournament = Tournament::new(vec![
        Warrior::new("Masarma", Weapon::new(WeaponKind::GreatSword)),
        Warrior::new("Lehtobel", Weapon::new(WeaponKind::Sword)),
        Warrior::new("Tendark", Weapon::new(WeaponKind::BattleAxe)),
        Warrior::new("Arcen", Weapon::new(WeaponKind::Sword)),
        Warrior::new("Morbiff", Weapon::new(WeaponKind::Axe)),
        Warrior::new("Nithu", Weapon::new(WeaponKind::WarHammer)),
        Warrior::new("Finul", Weapon::new(WeaponKind::Hammer)),
        Warrior::new("Chei", Weapon::new(WeaponKind::Hammer)),
    ]);

    tournament.fight_round(0);
}
