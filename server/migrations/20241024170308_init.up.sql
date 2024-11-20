-- Add up migration script here

-- Static Table with brand new weapons (shopping)
CREATE TABLE new_weapons (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    kind ENUM('Sword', 'GreatSword', 'Axe', 'BattleAxe', 'Hammer', 'WarHammer') NOT NULL,
    is_sharp BOOLEAN NOT NULL,
    is_two_handed BOOLEAN NOT NULL,
    rupture TINYINT UNSIGNED,
    additional_damages TINYINT UNSIGNED NOT NULL,
    attack_stat_modifier TINYINT NOT NULL,
    parry_stat_modifier TINYINT NOT NULL,
    courage_stat_modifier TINYINT NOT NULL
);

INSERT INTO new_weapons
    (name, kind, is_sharp, is_two_handed, rupture, additional_damages, attack_stat_modifier, parry_stat_modifier, courage_stat_modifier)
VALUES
    ("Shitty Sword", "Sword", 1, 0, 4, 3, 0, -1, -1),
    ("Basic Great Sword", "GreatSword", 1, 1, 4, 5, -3, -4, 0),
    ("Rusty Axe", "Axe", 1, 0, 3, 3, 0, -2, 0),
    ("Coarse Battle Axe", "BattleAxe", 1, 1, 3, 5, -3, -4, 0),
    ("Shitty Hammer", "Hammer", 0, 0, 4, 3, 0, -2, 0),
    ("Coarse War Hammer", "WarHammer", 0, 1, 4, 5, -3, -4, 0);

-- Static Table with brand new protections (shopping)
CREATE TABLE new_protections (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    kind ENUM('Armlets', 'Boots', 'Breastplate', 'ChainMail', 'Gambeson', 'Gloves', 'Greaves', 'Helm') NOT NULL,
    rupture TINYINT UNSIGNED,
    damage_reduction TINYINT UNSIGNED NOT NULL,
    dexterity_stat_modifier TINYINT NOT NULL,
    courage_stat_modifier TINYINT NOT NULL
);

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

-- Unique, mutable weapons
CREATE TABLE weapons (
    uuid CHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    kind ENUM('Sword', 'GreatSword', 'Axe', 'BattleAxe', 'Hammer', 'WarHammer') NOT NULL,
    is_sharp BOOLEAN NOT NULL,
    is_two_handed BOOLEAN NOT NULL,
    rupture TINYINT UNSIGNED,
    additional_damages TINYINT UNSIGNED NOT NULL,
    attack_stat_modifier TINYINT NOT NULL,
    parry_stat_modifier TINYINT NOT NULL,
    courage_stat_modifier TINYINT NOT NULL
);

-- Unique, mutable protections
CREATE TABLE protections (
    uuid CHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    kind ENUM('Armlets', 'Boots', 'Breastplate', 'ChainMail', 'Gambeson', 'Gloves', 'Greaves', 'Helm') NOT NULL,
    rupture TINYINT UNSIGNED,
    damage_reduction TINYINT UNSIGNED NOT NULL,
    dexterity_stat_modifier TINYINT NOT NULL,
    courage_stat_modifier TINYINT NOT NULL
);

-- Players
-- OWNS MANY warriors
CREATE TABLE players (
    uuid CHAR(36) PRIMARY KEY NOT NULL,
    username VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Players inventories
-- OWNS MANY slots
CREATE TABLE players_inventories (
    uuid CHAR(36) PRIMARY KEY NOT NULL,
    player_uuid CHAR(36) NOT NULL,
    gold INT UNSIGNED NOT NULL,
    CONSTRAINT FK_players_players_inventories FOREIGN KEY (player_uuid) REFERENCES players(uuid) ON DELETE CASCADE
);

-- Slots of a player's inventory
-- BELONGS TO ONE player's inventory
-- Optionally OWNS EITHER a weapon or protection
CREATE TABLE player_inventory_slots (
    uuid CHAR(36) PRIMARY KEY NOT NULL,
    inventory_uuid CHAR(36) NOT NULL,
    weapon_uuid CHAR(36),
    protection_uuid CHAR(36),
    CONSTRAINT FK_players_inventories_player_inventory_slots FOREIGN KEY (inventory_uuid) REFERENCES players_inventories(uuid) ON DELETE CASCADE
);

-- Warriors
-- OWNS MANY body parts
-- Optionally OWNS ONE weapon
-- BELONGS TO ONE player
CREATE TABLE warriors (
    uuid CHAR(36) NOT NULL PRIMARY KEY,
    player_uuid CHAR(36) NOT NULL,
    name VARCHAR(255) NOT NULL,
    current_health TINYINT UNSIGNED NOT NULL,
    max_health TINYINT UNSIGNED NOT NULL,
    tournament_uuid CHAR(36),
    nat_attack TINYINT UNSIGNED NOT NULL,
    nat_parry TINYINT UNSIGNED NOT NULL,
    nat_courage TINYINT UNSIGNED NOT NULL,
    nat_dexterity TINYINT UNSIGNED NOT NULL,
    nat_strength TINYINT UNSIGNED NOT NULL,
    last_passive_heal TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    experience BIGINT UNSIGNED NOT NULL DEFAULT 0,
    level TINYINT UNSIGNED NOT NULL DEFAULT 1,
    weapon_uuid CHAR(36),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    CONSTRAINT FK_players_warriors FOREIGN KEY (player_uuid) REFERENCES players(uuid) ON DELETE CASCADE
);

-- Body Parts
-- BELONGS TO ONE warrior
-- Optionally OWNS ONE protection
CREATE TABLE body_parts (
    uuid CHAR(36) PRIMARY KEY NOT NULL,
    warrior_uuid CHAR(36) NOT NULL,
    kind ENUM(
        'Head',
        'Left Eye',
        'Right Eye',
        'Torso',
        'Left Hand',
        'Right Hand',
        'Left Arm',
        'Right Arm',
        'Left Foot',
        'Right Foot',
        'Left Knee',
        'Right Knee',
        'Left Leg',
        'Right Leg',
        'Genitals',
        'Left Thumb',
        'Right Thumb',
        'Left Pointer Finger',
        'Right Pointer Finger',
        'Left Middle Finger',
        'Right Middle Finger',
        'Left Ring Finger',
        'Right Ring Finger',
        'Left Pinky Finger',
        'Right Pinky Finger'
    ) NOT NULL,
    is_broken BOOLEAN NOT NULL DEFAULT false,
    protection_uuid CHAR(36),
    CONSTRAINT FK_warriors_body_parts FOREIGN KEY (warrior_uuid) REFERENCES warriors(uuid) ON DELETE CASCADE
);

-- Tournaments
-- OWNS MANY tournaments players
CREATE TABLE tournaments (
    uuid CHAR(36) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    max_contestants TINYINT UNSIGNED NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    started_at TIMESTAMP
);

CREATE TABLE tournaments_warriors (
    uuid CHAR(36) NOT NULL PRIMARY KEY,
    tournament_uuid CHAR(36) NOT NULL,
    player_uuid CHAR(36) NOT NULL,
    warrior_uuid CHAR(36) NOT NULL,
    CONSTRAINT FK_tournaments_warriors_tournament_uuid FOREIGN KEY (tournament_uuid) REFERENCES tournaments(uuid) ON DELETE CASCADE,
    CONSTRAINT FK_tournaments_warriors_player_uuid FOREIGN KEY (player_uuid) REFERENCES players(uuid) ON DELETE CASCADE,
    CONSTRAINT FK_tournaments_warriors_warrior_uuid FOREIGN KEY (warrior_uuid) REFERENCES warriors(uuid) ON DELETE CASCADE
);
