CREATE TABLE IF NOT EXISTS game_query
(
    view_id text                        NOT NULL,
    version bigint CHECK (version >= 0) NOT NULL,
    payload jsonb                       NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE OR REPLACE TRIGGER notify_game_query_change
AFTER INSERT OR UPDATE OR DELETE ON game_query
FOR EACH ROW EXECUTE FUNCTION notify_change('game_query_change');
