DROP TRIGGER IF EXISTS notify_games_change ON games;

DROP TRIGGER IF EXISTS update_games_updated_at ON games;

DROP INDEX IF EXISTS idx_games_host_id;
DROP INDEX IF EXISTS idx_games_guest_id;

DROP TABLE IF EXISTS games;
