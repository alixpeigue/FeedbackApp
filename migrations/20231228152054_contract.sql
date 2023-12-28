CREATE TABLE contract (
    id SERIAL PRIMARY KEY,
    client INT REFERENCES client(id)
);

INSERT INTO contract (client) VALUES (1), (2);
