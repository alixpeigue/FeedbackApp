-- Add migration script here


-- Clients
INSERT INTO client (id, name) VALUES 
    (1, 'TCL'), 
    (2, 'RTE'), 
    (3, 'Enedis'),
    (4, 'Michelin');

-- Contracts
INSERT INTO contract (id, description, client) VALUES 
    (1, 'Entretien de matériel roulant', 1), 
    (2, 'Vérification des rails', 1), 
    (3, 'Inspection à vu de lignes électriques', 2), 
    (4, 'Entretien des cuves', 3), 
    (5, 'Maintenance péventive', 4);

-- Worders
INSERT INTO worker (id, name) VALUES 
    (1, 'Alix PEIGUE'),
    (2, 'Amar HASAN TAWFIQ'),
    (3, 'Ewan CHORYNSKI'),
    (4, 'Romain BENOIT'),
    (5, 'Théo CHONE'),
    (6, 'James SUDLOW'),
    (7, 'Bachir GUEDDOUDA');
    

-- Locations
INSERT INTO location (id, name) VALUES 
    (1, 'Entrepot Feyzin'), 
    (2, 'Vénissieux'), 
    (3, 'Villeurbanne'), 
    (4, 'Dépot de tram de la Doua'), 
    (5, 'Réacteur 2 de la centrale du Bugey'),
    (6, 'Lignes haute tension de la plaine de l''Ain'),
    (7, 'Usine Michelin de Blanzy');

INSERT INTO report (contract, worker, location, text) VALUES
    (2, 1, 2, 'Attention les rails supportent mal le froid.'),
    (2, 3, 3, 'TODO: Penser à vérifier le bon fonctionnement de la signalisation.'),
    (2, 3, 3, 'Travaux à Charpennes, trafique perturbé'),
    (3, 6, 6, 'Le code de l''armoir est: 1234 (pas très sécure)'),
    (4, 2, 5, 'TODO: Fuite récurrente, bien pensr à vérifier et réparer');
