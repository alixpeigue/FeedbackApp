CREATE TABLE report (
    id SERIAL PRIMARY KEY,
    contract INT REFERENCES contract(id),
    worker INT REFERENCES worker(id),
    location INT REFERENCES location(id),
    text TEXT,
    ts TSVECTOR GENERATED ALWAYS AS (TO_TSVECTOR('french', text)) STORED
);

CREATE INDEX ts_idx ON report USING GIN (ts);
