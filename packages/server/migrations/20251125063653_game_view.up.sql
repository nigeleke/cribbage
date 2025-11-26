CREATE OR REPLACE VIEW games AS
SELECT
    view_id::uuid AS id,
    COALESCE(payload -> 'instance' ->> 'name', 'Untitled')::text AS name,
    (payload -> 'instance' ->> 'host')::uuid AS host_id,
    (payload -> 'instance' ->> 'guest')::uuid AS guest_id,
    (payload -> 'instance' ->> 'state')::jsonb AS state
FROM game_query;
