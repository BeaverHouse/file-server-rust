CREATE ROLE admin WITH LOGIN PASSWORD 'test';

ALTER ROLE admin SUPERUSER;

DROP SCHEMA IF EXISTS family_app CASCADE;
CREATE SCHEMA family_app;

CREATE TABLE family_app.users
(
    id   SERIAL PRIMARY KEY,
    user_id  VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL    
);

CREATE TABLE family_app.alarms
(
    id   SERIAL PRIMARY KEY,
    user_id  VARCHAR(255) NOT NULL,
    file_path VARCHAR(255) NOT NULL,
    delete_flag VARCHAR(1) NOT NULL
);