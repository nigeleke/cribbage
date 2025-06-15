CREATE OR REPLACE VIEW available_games AS
        SELECT id, user_id1 AS user_id, 'Active' AS source, name, created_at
        FROM active_games
    UNION ALL
        SELECT id, user_id2 AS user_id, 'Active' AS source, name, created_at
        FROM active_games
    UNION ALL
        SELECT id, owner_id AS user_id, 'Lobby' AS source, name, created_at
        FROM lobby_games;
