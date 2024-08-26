-- A table for showing concurrency problems when updating row values
CREATE TABLE update_row (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    counter INTEGER NOT NULL DEFAULT 0
);
