CREATE TABLE contract (
    id SERIAL PRIMARY KEY,
    description TEXT,
    client INT REFERENCES client(id) NOT NULL
);

