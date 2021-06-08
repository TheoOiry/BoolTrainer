CREATE TABLE games
(
    id uuid,
    time_start timestamp without time zone NOT NULL,
    session_id uuid NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT session_id FOREIGN KEY (session_id)
        REFERENCES sessions (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
);
