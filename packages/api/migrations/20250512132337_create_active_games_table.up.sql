CREATE TABLE IF NOT EXISTS active_games
(
  id          UUID NOT NULL PRIMARY KEY,
  name        VARCHAR NOT NULL,
  host_id     UUID NOT NULL,
  guest_id    UUID NOT NULL,
  state       JSONB NOT NULL,
  created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_active_games_host_id ON active_games (host_id);
CREATE INDEX IF NOT EXISTS idx_active_games_guest_id ON active_games (guest_id);

CREATE OR REPLACE TRIGGER update_actives_games_updated_at
BEFORE UPDATE ON active_games
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE OR REPLACE TRIGGER active_games_notify
AFTER INSERT OR DELETE ON active_games
FOR EACH ROW EXECUTE FUNCTION notify_change('active_games');
