CREATE OR REPLACE TRIGGER unstarted_games_notify
AFTER INSERT OR DELETE ON unstarted_games
FOR EACH ROW EXECUTE FUNCTION notify_change('unstarted_games_change');
