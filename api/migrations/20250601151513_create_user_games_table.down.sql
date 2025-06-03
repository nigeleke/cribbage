DROP TRIGGER IF EXISTS user_games_notify ON user_games;
DROP TRIGGER IF EXISTS update_user_games_updated_at ON user_games;
DROP TRIGGER IF EXISTS trigger_insert_user_games ON active_games;

DROP FUNCTION IF EXISTS insert_user_games;

DROP TABLE IF EXISTS user_games;
