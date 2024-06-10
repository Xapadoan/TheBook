use the_book::horizontal_direction::HorizontalDirection;
use the_book::tournament::Tournament;
use the_book::warrior::body_parts::BodyPartKind;
use the_book::warrior::body_parts::WearProtection;
use the_book::warrior::protection::Protection;
use the_book::warrior::protection::ProtectionKind;
use the_book::warrior::Warrior;
use the_book::weapon::Weapon;
use the_book::weapon::WeaponKind;

fn main() {
    let mut masarma = Warrior::new("Masarma", Weapon::new(WeaponKind::GreatSword));
    let armlet = Protection::new(ProtectionKind::Armlet);
    if masarma.can_wear_protection(&armlet, BodyPartKind::Arm(HorizontalDirection::Left)) {
        masarma.wear_protection(armlet, BodyPartKind::Arm(HorizontalDirection::Left));
    }

    let mut lehtobel = Warrior::new("Lehtobel", Weapon::new(WeaponKind::Sword));
    let chain_mail = Protection::new(ProtectionKind::ChainMail);
    if lehtobel.can_wear_protection(&chain_mail, BodyPartKind::Torso) {
        lehtobel.wear_protection(chain_mail, BodyPartKind::Torso);
    }

    let mut tendark = Warrior::new("Tendark", Weapon::new(WeaponKind::BattleAxe));
    let helm = Protection::new(ProtectionKind::Helm);
    if tendark.can_wear_protection(&helm, BodyPartKind::Head) {
        tendark.wear_protection(helm, BodyPartKind::Head);
    }

    let mut arcen = Warrior::new("Arcen", Weapon::new(WeaponKind::Sword));
    let gambeson = Protection::new(ProtectionKind::Gambeson);
    if arcen.can_wear_protection(&gambeson, BodyPartKind::Torso) {
        arcen.wear_protection(gambeson, BodyPartKind::Torso);
    }

    let mut morbiff = Warrior::new("Morbiff", Weapon::new(WeaponKind::Axe));
    let greave = Protection::new(ProtectionKind::Greave);
    if morbiff.can_wear_protection(&greave, BodyPartKind::Leg(HorizontalDirection::Right)) {
        morbiff.wear_protection(greave, BodyPartKind::Leg(HorizontalDirection::Right));
    }

    let mut nithu = Warrior::new("Nithu", Weapon::new(WeaponKind::WarHammer));
    let jacket = Protection::new(ProtectionKind::Jacket);
    if nithu.can_wear_protection(&jacket, BodyPartKind::Torso) {
        nithu.wear_protection(jacket, BodyPartKind::Torso);
    }

    let mut finul = Warrior::new("Finul", Weapon::new(WeaponKind::Hammer));
    let plastron = Protection::new(ProtectionKind::Plastron);
    if finul.can_wear_protection(&plastron, BodyPartKind::Torso) {
        finul.wear_protection(plastron, BodyPartKind::Torso);
    }

    let mut chei = Warrior::new("Chei", Weapon::new(WeaponKind::Hammer));
    let helm = Protection::new(ProtectionKind::Helm);
    if chei.can_wear_protection(&helm, BodyPartKind::Head) {
        chei.wear_protection(helm, BodyPartKind::Head);
    }
    let mut tournament = Tournament::new(vec![
        masarma,
        lehtobel,
        tendark,
        arcen,
        morbiff,
        nithu,
        finul,
        chei,
    ]);

    tournament.fight_round(0);
}
