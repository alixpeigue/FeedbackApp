CREATE TABLE location (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

INSERT INTO location (name) VALUES ('Entrepot Feyzin'), ('Usine');
