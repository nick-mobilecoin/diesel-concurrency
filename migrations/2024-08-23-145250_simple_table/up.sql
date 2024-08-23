-- Your SQL goes here
CREATE TABLE simple_table (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL
    counter INTEGER NOT NULL DEFAULT 0,
);
