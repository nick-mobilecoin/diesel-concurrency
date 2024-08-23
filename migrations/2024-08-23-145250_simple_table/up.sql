-- Your SQL goes here
CREATE TABLE simple_table (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    counter INTEGER NOT NULL DEFAULT 0
);
