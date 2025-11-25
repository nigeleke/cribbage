CREATE TABLE IF NOT EXISTS account_query
(
    view_id text                        NOT NULL,
    version bigint CHECK (version >= 0) NOT NULL,
    payload json                        NOT NULL,
    PRIMARY KEY (view_id)
);
