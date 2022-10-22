ALTER TABLE
    todo DROP CONSTRAINT project_id_todo_number_same_nullability;

ALTER TABLE
    todo DROP CONSTRAINT project_id_todo_number_unique;

ALTER TABLE
    todo DROP COLUMN project_todo_number;

ALTER TABLE
    todo DROP COLUMN project_id;

DROP TABLE project;