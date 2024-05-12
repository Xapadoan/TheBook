use the_book::arena::Arena;
use the_book::warrior::Warrior;
use the_book::weapon::Weapon;
use the_book::weapon::WeaponKind;

fn main() {
    let mut arena = Arena::new(
        "Blue lagoon",
        Warrior::new("Masarma", Weapon::new(WeaponKind::GreatSword)),
        Warrior::new("Lehtobel", Weapon::new(WeaponKind::GreatSword)),
    );

    arena.fight();
}
