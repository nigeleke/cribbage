CREATE OR REPLACE FUNCTION notify_change() RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify(
        TG_ARGV[0],
        json_build_object(
            'operation', TG_OP,
            'table_name', TG_TABLE_NAME,
            'timing', TG_WHEN,
            'new_row', row_to_json(NEW),
            'old_row', row_to_json(OLD)
        )::text
    );
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
