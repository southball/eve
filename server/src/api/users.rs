use super::ErrorResponse;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

use crate::{
    auth::AccountId,
    session::{Session, SessionData},
};

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

#[derive(Serialize)]
struct PublicUser {
    username: String,
    display_name: String,
}

#[derive(Serialize)]
struct GetUserResponse {
    user: Option<PublicUser>,
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
