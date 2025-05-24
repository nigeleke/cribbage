CREATE TRIGGER started_games_notify
AFTER INSERT ON started_games
FOR EACH ROW EXECUTE FUNCTION notify_change('started_games_change');
