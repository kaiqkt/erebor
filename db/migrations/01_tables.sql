CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL CHECK (length(name) BETWEEN 1 AND 50),
    description VARCHAR(255) NOT NULL CHECK (
        length(description) <= 255
    )
);
