CREATE OR REPLACE TRIGGER active_games_notify
AFTER INSERT OR DELETE ON active_games
FOR EACH ROW EXECUTE FUNCTION notify_change('active_games_change');
