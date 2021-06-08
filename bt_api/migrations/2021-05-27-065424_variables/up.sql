CREATE TABLE public.variables
(
    id uuid,
    name character varying NOT NULL,
    value character varying NOT NULL,
    item_id uuid NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT item_id FOREIGN KEY (item_id)
        REFERENCES items (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
);