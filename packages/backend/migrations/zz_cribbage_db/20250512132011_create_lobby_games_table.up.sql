CREATE TABLE IF NOT EXISTS lobby_games
(
  id          UUID NOT NULL PRIMARY KEY,
  host_id     UUID NOT NULL,
  name        VARCHAR NOT NULL,
  created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_lobby_games_host_id ON lobby_games (host_id);

CREATE INDEX IF NOT EXISTS idx_lobby_games_created_at ON lobby_games (created_at);

CREATE OR REPLACE TRIGGER lobby_games_notify
AFTER INSERT OR DELETE ON lobby_games
FOR EACH ROW EXECUTE FUNCTION notify_change('lobby_games');
