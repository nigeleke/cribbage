CREATE OR REPLACE TRIGGER started_games_notify
AFTER INSERT OR DELETE ON started_games
FOR EACH ROW EXECUTE FUNCTION notify_change('started_games_change');
