CREATE TABLE cookie_session (
    id TEXT PRIMARY KEY NOT NULL,
    content TEXT NOT NULL,
    expiry TIMESTAMP NOT NULL
);
