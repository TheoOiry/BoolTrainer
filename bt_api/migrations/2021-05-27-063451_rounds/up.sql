CREATE TABLE rounds
(
    id uuid,
    expression character varying NOT NULL,
    game_id uuid NOT NULL,
    time_end timestamp without time zone,
    PRIMARY KEY (id),
    CONSTRAINT game_id FOREIGN KEY (game_id)
        REFERENCES games (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
);