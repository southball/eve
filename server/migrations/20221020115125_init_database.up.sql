CREATE TABLE account (
    id SERIAL PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    display_name TEXT NOT NULL,
    password_hash_and_salt TEXT NOT NULL
);

CREATE TABLE account_api_token (
    id SERIAL PRIMARY KEY,
    account_id INTEGER NOT NULL REFERENCES account(id),
    api_token TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expire_at TIMESTAMP NOT NULL
);

CREATE INDEX account_api_token_by_api_token ON account_api_token(api_token);

CREATE INDEX account_api_token_by_account_id ON account_api_token(account_id);

CREATE TABLE todo (
    id BIGSERIAL PRIMARY KEY,
    account_id INTEGER NOT NULL REFERENCES account(id),
    title TEXT NOT NULL,
    memo TEXT NOT NULL DEFAULT '',
    completed_at TIMESTAMP,
    deadline TIMESTAMP
);

CREATE INDEX todo_by_account_id ON todo (account_id);

CREATE TABLE todo_file (
    id BIGSERIAL,
    revision INTEGER DEFAULT 0,
    todo_id BIGINT NOT NULL REFERENCES todo(id) ON DELETE CASCADE,
    original_filename TEXT NOT NULL,
    memo TEXT NOT NULL DEFAULT '',
    file_accessor TEXT NOT NULL,
    PRIMARY KEY (id, revision)
);

CREATE INDEX todo_file_by_todo_id ON todo_file (todo_id);

CREATE TABLE tag (
    id BIGSERIAL PRIMARY KEY,
    account_id INTEGER REFERENCES account(id),
    tag_name TEXT NOT NULL
);

CREATE INDEX tag_by_account_id ON tag(account_id);

CREATE TABLE todo_tag (
    todo_id BIGINT REFERENCES todo(id) ON DELETE CASCADE,
    tag_id BIGINT REFERENCES tag(id) ON DELETE CASCADE 
);

CREATE INDEX todo_tag_by_todo_id ON todo_tag(todo_id);

CREATE INDEX todo_tag_by_tag_id ON todo_tag(tag_id);