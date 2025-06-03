CREATE TABLE IF NOT EXISTS unstarted_games
(
  id          UUID NOT NULL PRIMARY KEY,
  owner_id    UUID NOT NULL,
  name        VARCHAR NOT NULL,
  created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_unstarted_games_owner_id ON unstarted_games (owner_id);

CREATE INDEX IF NOT EXISTS idx_unstarted_games_created_at ON unstarted_games (created_at);

CREATE OR REPLACE TRIGGER unstarted_games_notify
AFTER INSERT OR DELETE ON unstarted_games
FOR EACH ROW EXECUTE FUNCTION notify_change('unstarted_games_change');
