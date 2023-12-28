CREATE TABLE client (
    id SERIAL PRIMARY KEY,
    name varchar(255) NOT NULL
);

INSERT INTO client (name) VALUES ('TCL'), ('OnePoint'), ('Esker');
