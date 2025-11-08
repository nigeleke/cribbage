CREATE OR REPLACE VIEW available_games AS
        SELECT id, host_id AS user_id, 'Active' AS source, name, created_at
        FROM games
        WHERE guest_id IS NOT NULL
    UNION ALL
        SELECT id, guest_id AS user_id, 'Active' AS source, name, created_at
        FROM games
        WHERE guest_id IS NOT NULL
    UNION ALL
        SELECT id, host_id AS user_id, 'Lobby' AS source, name, created_at
        FROM games
        WHERE guest_id IS NULL;
