CREATE TABLE IF NOT EXISTS active_games
(
  id          UUID NOT NULL PRIMARY KEY,
  name        VARCHAR NOT NULL,
  user_id1    UUID NOT NULL,
  user_id2    UUID NOT NULL,
  state       JSONB NOT NULL,
  created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_active_games_user_id1 ON active_games (user_id1);
CREATE INDEX IF NOT EXISTS idx_active_games_user_id2 ON active_games (user_id2);

CREATE OR REPLACE TRIGGER update_actives_games_updated_at
BEFORE UPDATE ON active_games
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE OR REPLACE TRIGGER active_games_notify
AFTER INSERT OR DELETE ON active_games
FOR EACH ROW EXECUTE FUNCTION notify_change('active_games_change');
