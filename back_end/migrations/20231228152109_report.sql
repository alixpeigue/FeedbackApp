CREATE TABLE report (
    id SERIAL PRIMARY KEY,
    contract INT REFERENCES contract(id) NOT NULL,
    worker INT REFERENCES worker(id) NOT NULL,
    location INT REFERENCES location(id) NOT NULL,
    text TEXT NOT NULL, 
    ts TSVECTOR GENERATED ALWAYS AS (TO_TSVECTOR('french', text)) STORED
);

CREATE INDEX ts_idx ON report USING GIN (ts);
