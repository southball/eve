use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use sqlx::postgres::PgPool;

use crate::auth::AccountId;

pub async fn get_todos(
    AccountId(account_id): AccountId,
    Extension(pg_pool): Extension<PgPool>,
) -> impl IntoResponse {
    let todos = sqlx::query!("SELECT * FROM todo WHERE account_id = $1", account_id)
        .fetch_all(&pg_pool)
        .await
        .unwrap()
        .into_iter()
        .map(|record| record.title)
        .collect::<Vec<_>>();

    Json(todos)
}

#[derive(Deserialize)]
pub struct CreateTodoRequest;

pub async fn post_todos(
    AccountId(_account_id): AccountId,
    Extension(_pg_pool): Extension<PgPool>,
    Json(_req): Json<CreateTodoRequest>,
) -> impl IntoResponse {
    unimplemented!()
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest;

pub async fn post_todo(
    AccountId(_account_id): AccountId,
    Extension(_pg_pool): Extension<PgPool>,
    Path(_todo_id): Path<String>,
    Json(_req): Json<UpdateTodoRequest>,
) -> impl IntoResponse {
    unimplemented!()
}
