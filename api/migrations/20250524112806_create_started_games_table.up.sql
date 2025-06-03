CREATE TABLE IF NOT EXISTS started_games
(
  unstarted_game_id UUID NOT NULL PRIMARY KEY,
  active_game_id    UUID NOT NULL,
  created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_started_games_active_game_id ON started_games (active_game_id);
CREATE INDEX IF NOT EXISTS idx_started_games_created_at ON started_games (created_at);

CREATE OR REPLACE TRIGGER started_games_notify
AFTER INSERT OR DELETE ON started_games
FOR EACH ROW EXECUTE FUNCTION notify_change('started_games_change');
