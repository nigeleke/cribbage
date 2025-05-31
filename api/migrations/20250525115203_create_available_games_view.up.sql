CREATE OR REPLACE VIEW available_games AS

    SELECT
        game_id as id,
        user_id,
        'Active' AS source,
        name,
        created_at
    FROM user_games

UNION

    SELECT
        id,
        owner_id AS user_id,
        'Unstarted' AS source,
        name,
        created_at
    FROM unstarted_games;
