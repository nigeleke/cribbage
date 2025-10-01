CREATE OR REPLACE VIEW available_games AS
        SELECT id, host_id AS user_id, 'Active' AS source, name, created_at
        FROM active_games
    UNION ALL
        SELECT id, guest_id AS user_id, 'Active' AS source, name, created_at
        FROM active_games
    UNION ALL
        SELECT id, host_id AS user_id, 'Lobby' AS source, name, created_at
        FROM lobby_games;
