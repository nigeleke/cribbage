CREATE TABLE IF NOT EXISTS user_games
(
    game_id     UUID NOT NULL REFERENCES active_games(id) ON DELETE CASCADE,
    user_id     UUID NOT NULL,
    state       JSONB NOT NULL,
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (game_id, user_id)
);

CREATE OR REPLACE FUNCTION insert_user_games()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO user_games (game_id, user_id, state, created_at, updated_at)
    VALUES (NEW.id, NEW.user_id1, NEW.state, NEW.created_at, NEW.updated_at);

    INSERT INTO user_games (game_id, user_id, state, created_at, updated_at)
    VALUES (NEW.id, NEW.user_id2, NEW.state, NEW.created_at, NEW.updated_at);

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trigger_insert_user_games
AFTER INSERT ON active_games
FOR EACH ROW
EXECUTE FUNCTION insert_user_games();

CREATE OR REPLACE TRIGGER update_user_games_updated_at
BEFORE UPDATE ON user_games
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE OR REPLACE TRIGGER user_games_notify
AFTER UPDATE ON user_games
FOR EACH ROW EXECUTE FUNCTION notify_change('user_games_change');
