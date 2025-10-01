DROP TRIGGER IF EXISTS lobby_games_notify ON lobby_games;

DROP INDEX IF EXISTS idx_lobby_games_host_id;
DROP INDEX IF EXISTS idx_lobby_games_created_at;

DROP TABLE IF EXISTS lobby_games;
