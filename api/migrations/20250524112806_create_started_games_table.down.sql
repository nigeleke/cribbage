DROP TRIGGER IF EXISTS started_games_notify ON started_games;

DROP INDEX IF EXISTS idx_started_games_active_game_id;
DROP INDEX IF EXISTS idx_started_games_created_at;

DROP TABLE IF EXISTS started_games;
