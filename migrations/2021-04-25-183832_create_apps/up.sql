CREATE TABLE apps (
    id INTEGER NOT NULL PRIMARY KEY,
    key TEXT NOT NULL UNIQUE,
    secret TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL
);
