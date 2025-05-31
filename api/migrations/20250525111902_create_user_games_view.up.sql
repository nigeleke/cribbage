CREATE OR REPLACE VIEW user_games AS
    SELECT
        user_id1 AS user_id,
        id AS game_id,
        name,
        created_at
    FROM active_games
UNION
    SELECT
        user_id2 AS user_id,
        id AS game_id,
        name,
        created_at
    FROM active_games;
