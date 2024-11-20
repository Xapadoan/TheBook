INSERT INTO tournaments (uuid, name, max_contestants) VALUES (
    "00000000-0000-0000-0000-000000000000",
    "Open Tournament",
    8
);

INSERT INTO tournaments_warriors (uuid, tournament_uuid, player_uuid, warrior_uuid)
VALUES (
    "00000000-0000-0000-0000-000000000000",
    "00000000-0000-0000-0000-000000000000",
    "00000000-0000-0000-0000-000000000000"
);

UPDATE warriors
SET tournament_uuid = "00000000-0000-0000-0000-000000000000"
WHERE uuid = "00000000-0000-0000-0000-000000000000";
