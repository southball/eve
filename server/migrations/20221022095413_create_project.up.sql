CREATE TABLE project (
    id BIGSERIAL PRIMARY KEY,
    account_id INTEGER REFERENCES account(id) ON DELETE CASCADE,
    shortcode TEXT NOT NULL,
    project_name TEXT NOT NULL,
    UNIQUE(account_id, shortcode)
);

ALTER TABLE
    todo
ADD
    COLUMN project_id BIGINT REFERENCES project(id);

ALTER TABLE
    todo
ADD
    COLUMN project_todo_number INT;

ALTER TABLE
    todo
ADD
    CONSTRAINT project_id_todo_number_unique UNIQUE(project_id, project_todo_number);

ALTER TABLE
    todo
ADD
    CONSTRAINT project_id_todo_number_same_nullability CHECK(
        (
            project_id IS NOT NULL
            AND project_todo_number IS NOT NULL
        )
        OR (
            project_id IS NULL
            AND project_todo_number IS NULL
        )
    );