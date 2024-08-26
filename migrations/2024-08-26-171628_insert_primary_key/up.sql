-- Your SQL goes here
CREATE TABLE serial_key_table (
    id SERIAL PRIMARY KEY,
    some_value INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE uuid_key_table (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    some_value INTEGER NOT NULL DEFAULT 0
);