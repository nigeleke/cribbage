CREATE TABLE IF NOT EXISTS unstarted_games
(
  id          UUID NOT NULL PRIMARY KEY,
  owner_id    UUID NOT NULL,
  name        VARCHAR NOT NULL,
  created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_unstarted_games_created_at ON unstarted_games (created_at);
