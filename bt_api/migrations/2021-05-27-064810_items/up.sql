CREATE TABLE items
(
    id uuid,
    expected boolean NOT NULL,
    found boolean,
    round_id uuid NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT round_id FOREIGN KEY (round_id)
        REFERENCES rounds (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
);