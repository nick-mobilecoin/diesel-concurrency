-- Your SQL goes here
CREATE TABLE serial_key_table (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    some_value INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE uuid_key_table (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    some_value INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE unique_column_table (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    some_value INTEGER NOT NULL UNIQUE
);

CREATE TABLE unique_string_column_table (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    some_value VARCHAR(255) NOT NULL UNIQUE
);
