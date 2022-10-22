use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

use crate::auth::AccountId;

use super::{APIResponse, APIResult, ErrorResponse};

#[derive(Serialize)]
pub struct PublicTodo {
    id: i64,
    title: String,
    memo: String,
    completed_at: Option<DateTime<Utc>>,
    deadline: Option<DateTime<Utc>>,
    project_id: Option<i64>,
    project_todo_number: Option<i32>,
}

pub async fn get_todos(
    AccountId(account_id): AccountId,
    Extension(pg_pool): Extension<PgPool>,
) -> impl IntoResponse {
    let todos = sqlx::query!("SELECT * FROM todo WHERE account_id = $1", account_id)
        .fetch_all(&pg_pool)
        .await
        .unwrap()
        .into_iter()
        .map(|record| PublicTodo {
            id: record.id,
            title: record.title,
            memo: record.memo,
            project_id: record.project_id,
            project_todo_number: record.project_todo_number,
            completed_at: record
                .completed_at
                .map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc)),
            deadline: record
                .deadline
                .map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc)),
        })
        .collect::<Vec<_>>();

    Json(todos)
}

#[derive(Deserialize)]
pub struct CreateOrUpdateTodoRequest {
    title: String,
    memo: Option<String>,
    completed_at: Option<DateTime<Utc>>,
    deadline: Option<DateTime<Utc>>,
    project_id: Option<i64>,
}

async fn validate_account_has_project(
    pg_pool: &PgPool,
    account_id: i32,
    project_id: i64,
) -> APIResult<()> {
    let count = sqlx::query!(
        "
            SELECT COUNT(*) FROM project
            WHERE id = $1 and account_id = $2
        ",
        project_id,
        account_id
    )
    .fetch_one(pg_pool)
    .await
    .map(|row| row.count.unwrap_or(0))
    .map_err(|_err| {
        ErrorResponse::from(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to validate project.",
        )
    })?;

    if count == 0 {
        return Err(ErrorResponse::from(
            StatusCode::UNAUTHORIZED,
            "Project does not exist.",
        ));
    }

    Ok(())
}

async fn validate_account_has_todo(
    pg_pool: &PgPool,
    account_id: i32,
    todo_id: i64,
) -> APIResult<()> {
    let count = sqlx::query!(
        "
            SELECT COUNT(*) FROM todo
            WHERE id = $1 AND account_id = $2
        ",
        todo_id,
        account_id
    )
    .fetch_one(pg_pool)
    .await
    .map(|record| record.count)
    .map_err(|_err| {
        ErrorResponse::from(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to validate todo belongs to user.",
        )
    })?
    .unwrap_or(0);

    if count > 0 {
        Ok(())
    } else {
        Err(ErrorResponse::from(
            StatusCode::NOT_FOUND,
            "The todo does not exist.",
        ))
    }
}

pub async fn post_todos(
    AccountId(account_id): AccountId,
    Extension(pg_pool): Extension<PgPool>,
    Json(req): Json<CreateOrUpdateTodoRequest>,
) -> APIResponse<PublicTodo> {
    if let Some(project_id) = req.project_id {
        validate_account_has_project(&pg_pool, account_id, project_id).await?;
    }

    let record = sqlx::query!(
        "
            INSERT INTO todo (account_id, title, memo, completed_at, deadline, project_id, project_todo_number)
            VALUES (
                $1, $2, $3, $4, $5, CAST($6 AS BIGINT),
                CASE WHEN $6 IS NULL THEN NULL ELSE (
                    SELECT COALESCE(MAX(project_todo_number), 0) + 1 FROM todo
                    WHERE project_id = $6
                ) END
            )
            RETURNING *
        ",
        &account_id,
        &req.title,
        &req.memo.unwrap_or_else(|| "".to_string()),
        req.completed_at.map(|datetime| datetime.naive_utc()),
        req.deadline.map(|datetime| datetime.naive_utc()),
        req.project_id
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create todo."))?;

    Ok(PublicTodo {
        id: record.id,
        title: record.title,
        memo: record.memo,
        project_id: record.project_id,
        project_todo_number: record.project_todo_number,
        completed_at: record
            .completed_at
            .map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc)),
        deadline: record
            .deadline
            .map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc)),
    }
    .into())
}

pub async fn post_todo(
    AccountId(account_id): AccountId,
    Extension(pg_pool): Extension<PgPool>,
    Path(todo_id): Path<i64>,
    Json(req): Json<CreateOrUpdateTodoRequest>,
) -> APIResponse<PublicTodo> {
    if let Some(project_id) = req.project_id {
        validate_account_has_project(&pg_pool, account_id, project_id).await?;
    }

    validate_account_has_todo(&pg_pool, account_id, todo_id).await?;

    let record = sqlx::query!(
        "
            UPDATE todo
            SET title = $2,
                memo = $3,
                completed_at = $4,
                deadline = $5,
                project_id = CAST($6 AS BIGINT),
                project_todo_number =
                    CASE WHEN project_id = $6 THEN project_todo_number ELSE
                    CASE WHEN $6 IS NULL THEN NULL ELSE
                    (
                        SELECT COALESCE(MAX(project_todo_number), 0) + 1 FROM todo
                        WHERE project_id = $6
                    ) END END
            WHERE id = $1
            RETURNING *
        ",
        todo_id,
        &req.title,
        &req.memo.unwrap_or_else(|| "".to_string()),
        req.completed_at.map(|datetime| datetime.naive_utc()),
        req.deadline.map(|datetime| datetime.naive_utc()),
        req.project_id
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(|_| {
        ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create todo.")
    })?;

    Ok(PublicTodo {
        id: record.id,
        title: record.title,
        memo: record.memo,
        project_id: record.project_id,
        project_todo_number: record.project_todo_number,
        completed_at: record
            .completed_at
            .map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc)),
        deadline: record
            .deadline
            .map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc)),
    }
    .into())
}
