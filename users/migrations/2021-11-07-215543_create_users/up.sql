-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(128) NOT NULL,
    password VARCHAR(256) NOT NULL
);
