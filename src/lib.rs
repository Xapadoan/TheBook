mod dice;
mod fight;
mod modifiers;
mod equipment;
mod tournament;
mod warrior;
mod virtual_timer;
mod name;

use std::error::Error;

use warrior::Warrior;
use warrior::weapon::{Weapon, WeaponKind, MayHaveWeapon, GiveWeapon};
use warrior::protection::{Protection, ProtectionKind, WearProtection};
use warrior::body::body_part::BodyPartKind;
use warrior::body::body_side::BodySide;
use tournament::Tournament;

pub fn run() -> Result<(), Box<dyn Error>> {
let mut masarma = Warrior::new("Masarma");
    let great_sword = Weapon::new(WeaponKind::GreatSword);
    if masarma.weapon().is_none() {
        masarma.give_weapon(great_sword);
    }
    let armlet = Protection::new(ProtectionKind::Armlet);
    if masarma.can_wear_protection(&armlet, BodyPartKind::Arm(BodySide::Left)) {
        masarma.wear_protection(armlet, BodyPartKind::Arm(BodySide::Left));
    }
    let boot = Protection::new(ProtectionKind::Boot);
    if masarma.can_wear_protection(&boot, BodyPartKind::Foot(BodySide::Right)) {
        masarma.wear_protection(boot, BodyPartKind::Foot(BodySide::Right));
    }

    let mut lehtobel = Warrior::new("Lehtobel");
    let sword = Weapon::new(WeaponKind::Sword);
    if lehtobel.weapon().is_none() {
        lehtobel.give_weapon(sword);
    }
    let chain_mail = Protection::new(ProtectionKind::ChainMail);
    if lehtobel.can_wear_protection(&chain_mail, BodyPartKind::Torso) {
        lehtobel.wear_protection(chain_mail, BodyPartKind::Torso);
    }

    let mut tendark = Warrior::new("Tendark");
    let battle_axe = Weapon::new(WeaponKind::BattleAxe);
    if tendark.weapon().is_none() {
        tendark.give_weapon(battle_axe);
    }
    let gauntlet = Protection::new(ProtectionKind::Gauntlet);
    if tendark.can_wear_protection(&gauntlet, BodyPartKind::Hand(BodySide::Right)) {
        tendark.wear_protection(gauntlet, BodyPartKind::Hand(BodySide::Right));
    }
    let helm = Protection::new(ProtectionKind::Helm);
    if tendark.can_wear_protection(&helm, BodyPartKind::Head) {
        tendark.wear_protection(helm, BodyPartKind::Head);
    }

    let mut arcen = Warrior::new("Arcen");
    let sword = Weapon::new(WeaponKind::Sword);
    if arcen.weapon().is_none() {
        arcen.give_weapon(sword);
    }
    let gambeson = Protection::new(ProtectionKind::Gambeson);
    if arcen.can_wear_protection(&gambeson, BodyPartKind::Torso) {
        arcen.wear_protection(gambeson, BodyPartKind::Torso);
    }

    let mut morbiff = Warrior::new("Morbiff");
    let axe = Weapon::new(WeaponKind::Axe);
    if morbiff.weapon().is_none() {
        morbiff.give_weapon(axe);
    }
    let greave = Protection::new(ProtectionKind::Greave);
    let gauntlet = Protection::new(ProtectionKind::Gauntlet);
    if morbiff.can_wear_protection(&greave, BodyPartKind::Leg(BodySide::Right)) {
        morbiff.wear_protection(greave, BodyPartKind::Leg(BodySide::Right));
    }
    if morbiff.can_wear_protection(&gauntlet, BodyPartKind::Hand(BodySide::Left)) {
        morbiff.wear_protection(gauntlet, BodyPartKind::Hand(BodySide::Left))
    }

    let mut nithu = Warrior::new("Nithu");
    let war_hammer = Weapon::new(WeaponKind::WarHammer);
    if nithu.weapon().is_none() {
        nithu.give_weapon(war_hammer);
    }
    let jacket = Protection::new(ProtectionKind::Jacket);
    if nithu.can_wear_protection(&jacket, BodyPartKind::Torso) {
        nithu.wear_protection(jacket, BodyPartKind::Torso);
    }

    let mut finul = Warrior::new("Finul");
    let hammer = Weapon::new(WeaponKind::Hammer);
    if finul.weapon().is_none() {
        finul.give_weapon(hammer);
    }
    let plastron = Protection::new(ProtectionKind::Plastron);
    if finul.can_wear_protection(&plastron, BodyPartKind::Torso) {
        finul.wear_protection(plastron, BodyPartKind::Torso);
    }

    let mut chei = Warrior::new("Chei");
    let hammer = Weapon::new(WeaponKind::Hammer);
    if chei.weapon().is_none() {
        chei.give_weapon(hammer);
    }
    let helm = Protection::new(ProtectionKind::Helm);
    if chei.can_wear_protection(&helm, BodyPartKind::Head) {
        chei.wear_protection(helm, BodyPartKind::Head);
    }
    let boot = Protection::new(ProtectionKind::Boot);
    if chei.can_wear_protection(&boot, BodyPartKind::Foot(BodySide::Left)) {
        chei.wear_protection(boot, BodyPartKind::Foot(BodySide::Left));
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
    Ok(())
}
