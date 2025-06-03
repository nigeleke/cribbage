DROP TRIGGER IF EXISTS unstarted_games_notify ON unstarted_games;

DROP INDEX IF EXISTS idx_unstarted_games_owner_id;
DROP INDEX IF EXISTS idx_unstarted_games_created_at;

DROP TABLE IF EXISTS unstarted_games;
