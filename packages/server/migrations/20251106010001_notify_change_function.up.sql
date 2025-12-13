CREATE OR REPLACE FUNCTION notify_change()
RETURNS trigger
LANGUAGE plpgsql AS $$
DECLARE
    pk jsonb;
    payload jsonb;
    row_data jsonb;
BEGIN
    row_data :=
        CASE
            WHEN TG_OP = 'DELETE' THEN to_jsonb(OLD)
            ELSE to_jsonb(NEW)
        END;

    SELECT jsonb_agg(
               jsonb_build_object(
                   'column', a.attname,
                   'value',  row_data -> a.attname
               )
               ORDER BY x.ordinality
           )
    INTO pk
    FROM pg_index i
    JOIN unnest(i.indkey) WITH ORDINALITY AS x(attnum, ordinality)
      ON TRUE
    JOIN pg_attribute a
      ON a.attrelid = i.indrelid
     AND a.attnum   = x.attnum
    WHERE i.indrelid = TG_RELID
      AND i.indisprimary;

    payload := jsonb_build_object(
        'operation',   TG_OP,
        'schema',      TG_TABLE_SCHEMA,
        'table_name',  TG_TABLE_NAME,
        'timing',      TG_WHEN,
        'primary_key', COALESCE(pk, '[]'::jsonb)
    );

    PERFORM pg_notify(TG_ARGV[0], payload::text);
    RETURN NULL;
END;
$$;
