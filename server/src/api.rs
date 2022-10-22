use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

use crate::{
    auth::AccountId,
    session::{Session, SessionData},
};

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
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn post_login(
    Extension(pg_pool): Extension<PgPool>,
    Json(req): Json<LoginRequest>,
    mut session: Session,
) -> impl IntoResponse {
    let account_row = sqlx::query!(
        "
            SELECT * FROM account
            WHERE username = $1
        ",
        &req.username
    )
    .fetch_optional(&pg_pool)
    .await
    .unwrap();

    let failed_response = (StatusCode::UNAUTHORIZED, "Wrong username or password.");

    match account_row {
        None => failed_response,
        Some(account) => {
            if !bcrypt::verify(&req.password, &account.password_hash_and_salt).unwrap() {
                return failed_response;
            }
            session
                .set(SessionData {
                    account_id: Some(account.id),
                })
                .await
                .unwrap();
            (StatusCode::OK, "Logged in.")
        }
    }
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

#[derive(Serialize)]
struct PublicUser {
    username: String,
    display_name: String,
}

#[derive(Serialize)]
struct GetUserResponse {
    user: Option<PublicUser>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error_message: String,
}

impl From<&str> for ErrorResponse {
    fn from(error_message: &str) -> Self {
        ErrorResponse {
            error_message: error_message.to_owned(),
        }
    }
}

pub async fn get_user(
    account_id: Option<AccountId>,
    Extension(pg_pool): Extension<PgPool>,
) -> impl IntoResponse {
    let empty_response = Ok(Json(GetUserResponse { user: None }));

    let account_id = match account_id {
        Some(AccountId(account_id)) => account_id,
        None => return empty_response,
    };

    sqlx::query!(
        "
            SELECT username, display_name FROM account
            WHERE id = $1
        ",
        &account_id
    )
    .fetch_optional(&pg_pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::from("Failed to fetch from database.")),
        )
    })
    .and_then(|result| match result {
        Some(result) => Ok(result),
        None => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::from("Invalid account.")),
        )),
    })
    .map(|user| {
        Json(GetUserResponse {
            user: Some(PublicUser {
                username: user.username,
                display_name: user.display_name,
            }),
        })
    })
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    username: String,
    display_name: Option<String>,
    password: String,
}

const BCRYPT_COST: u32 = 10;

pub async fn post_users(
    Extension(pg_pool): Extension<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let hash_result = bcrypt::hash(req.password, BCRYPT_COST).unwrap();
    let same_username_result = sqlx::query!(
        "SELECT * FROM account WHERE username = $1 LIMIT 1",
        &req.username
    )
    .fetch_all(&pg_pool)
    .await
    .unwrap();
    if !same_username_result.is_empty() {
        return (StatusCode::BAD_REQUEST, "Username is already used.");
    }
    let _create_user_result = sqlx::query!(
        "
                INSERT INTO account (username, display_name, password_hash_and_salt)
                VALUES ($1, $2, $3)
            ",
        &req.username,
        req.display_name.as_ref().unwrap_or(&req.username),
        &hash_result
    )
    .execute(&pg_pool)
    .await;
    (StatusCode::OK, "User is created.")
}
