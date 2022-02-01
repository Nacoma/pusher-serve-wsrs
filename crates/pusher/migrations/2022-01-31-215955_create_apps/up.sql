CREATE TABLE apps
(
    id     bigint unsigned primary key not null,
    name   TEXT NOT NULL,
    `key`  TEXT NOT NULL,
    secret TEXT NOT NULL
);


