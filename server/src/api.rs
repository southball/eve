use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx::postgres::PgPool;

use crate::auth::AccountId;

pub fn get_api_router() -> Router {
    Router::new()
        .route("/user", get(get_user))
        .route("/users", post(post_users))
        .route("/login", post(post_login))
        .route("/todos", get(get_todos).post(post_todos))
        .route("/todo/:todo_id", post(post_todo))
        .route("/todo/:todo_id/files", post(post_todo_files))
        .route("/files", get(get_files).post(post_files))
        .route("/file/:file_id", get(get_file).post(post_file))
}

pub async fn post_todo_files(
    Extension(_pg_pool): Extension<PgPool>,
    Path(_todo_id): Path<String>,
    AccountId(_account_id): AccountId,
) -> impl IntoResponse {
    unimplemented!()
}

pub async fn post_files(
    Extension(_pg_pool): Extension<PgPool>,
    AccountId(_account_id): AccountId,
) -> impl IntoResponse {
    unimplemented!()
}

pub async fn post_file(
    Extension(_pg_pool): Extension<PgPool>,
    AccountId(_account_id): AccountId,
) -> impl IntoResponse {
    unimplemented!()
}

pub async fn get_files(
    Extension(_pg_pool): Extension<PgPool>,
    AccountId(_account_id): AccountId,
) -> impl IntoResponse {
    unimplemented!()
}

pub async fn get_file(
    Extension(_pg_pool): Extension<PgPool>,
    Path(_file_id): Path<String>,
) -> impl IntoResponse {
    unimplemented!()
}

#[derive(Deserialize)]
pub struct LoginRequest;

pub async fn post_login(
    Extension(_pg_pool): Extension<PgPool>,
    Json(_req): Json<LoginRequest>,
) -> impl IntoResponse {
    unimplemented!()
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

pub async fn get_user(
    AccountId(_account_id): AccountId,
    Extension(_pg_pool): Extension<PgPool>,
) -> impl IntoResponse {
    unimplemented!()
}

#[derive(Deserialize)]
pub struct CreateUserRequest;

pub async fn post_users(
    Extension(_pg_pool): Extension<PgPool>,
    Json(_req): Json<CreateUserRequest>,
) -> impl IntoResponse {
    unimplemented!()
}
