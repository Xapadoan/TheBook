USE the_book;

DELETE FROM new_weapons;
INSERT INTO new_weapons
    (name, kind, is_sharp, is_two_handed, rupture, additional_damages, attack_stat_modifier, parry_stat_modifier, courage_stat_modifier)
VALUES
    ("Shitty Sword", "Sword", 1, 0, 4, 3, 0, -1, -1),
    ("Basic Great Sword", "GreatSword", 1, 1, 4, 5, -3, -4, 0),
    ("Rusty Axe", "Axe", 1, 0, 3, 3, 0, -2, 0),
    ("Coarse Battle Axe", "BattleAxe", 1, 1, 3, 5, -3, -4, 0),
    ("Shitty Hammer", "Hammer", 0, 0, 4, 3, 0, -2, 0),
    ("Coarse War Hammer", "WarHammer", 0, 1, 4, 5, -3, -4, 0);

DELETE FROM new_protections;
INSERT INTO new_protections
    (name, kind, rupture, damage_reduction, courage_stat_modifier, dexterity_stat_modifier)
VALUES
    ("Shabby leather boots", "Boots", 5, 0, 0, 0),
    ("Basic leather breastplate", "Breastplate", 4, 3, 0, 0),
    ("Rusty chain mail", "ChainMail", 4, 3, 0, -1),
    ("Basic gambeson", "Gambeson", 4, 2, 0, 0),
    ("Leather Gloves", "Gloves", 5, 0, 0, 0),
    ("Heavy coarse greaves", "Greaves", 5, 1, 0, -2),
    ("Shabby leather helmet", "Helm", 5, 0, 0, 0),
    ("Heavy coarse metal armlet", "Armlets", 5, 1, 0, -2);
