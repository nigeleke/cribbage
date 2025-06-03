DROP TRIGGER IF EXISTS active_games_notify ON active_games;

DROP TRIGGER IF EXISTS update_actives_games_updated_at ON active_games;

DROP INDEX IF EXISTS idx_active_games_user_id1;
DROP INDEX IF EXISTS idx_active_games_user_id2;

DROP TABLE IF EXISTS active_games;
