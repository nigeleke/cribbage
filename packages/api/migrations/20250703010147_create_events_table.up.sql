CREATE TABLE events
(
    aggregate_type TEXT                         NOT NULL,
    aggregate_id   TEXT                         NOT NULL,
    sequence       BIGINT CHECK (sequence >= 0) NOT NULL,
    event_type     TEXT                         NOT NULL,
    event_version  TEXT                         NOT NULL,
    payload        JSON                         NOT NULL,
    metadata       JSON                         NOT NULL,
    timestamp      TIMESTAMP WITH TIME ZONE DEFAULT (CURRENT_TIMESTAMP),
    PRIMARY KEY (aggregate_type, aggregate_id, sequence)
);
