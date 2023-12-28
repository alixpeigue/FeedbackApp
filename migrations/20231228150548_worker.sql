CREATE TABLE worker (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

INSERT INTO worker (name) VALUES ('Jean DUPUIS'), ('Céline KRAPIVIC');
