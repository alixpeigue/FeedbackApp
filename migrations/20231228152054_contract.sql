CREATE TABLE contract (
    id SERIAL PRIMARY KEY,
    description TEXT,
    client INT REFERENCES client(id) NOT NULL
);

INSERT INTO contract (description, client) VALUES ('entretien de matériel roulant', 1), (NULL, 2);
