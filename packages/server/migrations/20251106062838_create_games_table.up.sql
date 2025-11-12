CREATE TABLE IF NOT EXISTS games
(
  id          UUID NOT NULL PRIMARY KEY DEFAULT uuidv7(),
  name        VARCHAR NOT NULL,
  host_id     UUID NOT NULL,
  guest_id    UUID,
  state       JSONB NOT NULL,
  created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_games_host_id ON games (host_id);
CREATE INDEX IF NOT EXISTS idx_games_guest_id ON games (guest_id);

CREATE OR REPLACE TRIGGER update_games_updated_at
BEFORE UPDATE ON games
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE OR REPLACE TRIGGER notify_games_change
AFTER INSERT OR UPDATE OR DELETE ON games
FOR EACH ROW EXECUTE FUNCTION notify_change('games_change');
