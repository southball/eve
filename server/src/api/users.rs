use super::{APIResponse, ErrorResponse};
use axum::{extract::Extension, http::StatusCode, Json};
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
) -> APIResponse<()> {
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

    let failed_response = Err(ErrorResponse::from(
        StatusCode::UNAUTHORIZED,
        "Invalid username or password.",
    ));

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
            Ok(().into())
        }
    }
}

#[derive(Serialize)]
pub struct PublicUser {
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
) -> APIResponse<Option<PublicUser>> {
    let account_id = match account_id {
        Some(AccountId(account_id)) => account_id,
        None => return Ok(None.into()),
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
        ErrorResponse::from(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch from database.",
        )
    })
    .and_then(|result| match result {
        Some(result) => Ok(result),
        None => Err(ErrorResponse::from(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid account.",
        )),
    })
    .map(|user| {
        Some(PublicUser {
            username: user.username,
            display_name: user.display_name,
        })
        .into()
    })
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    username: String,
    display_name: Option<String>,
    password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    username: Option<String>,
    display_name: Option<String>,
    password: Option<String>,
}

const BCRYPT_COST: u32 = 10;

pub async fn post_users(
    Extension(pg_pool): Extension<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> APIResponse<PublicUser> {
    let hash_result = bcrypt::hash(req.password, BCRYPT_COST).unwrap();
    let same_username_result = sqlx::query!(
        "SELECT * FROM account WHERE username = $1 LIMIT 1",
        &req.username
    )
    .fetch_optional(&pg_pool)
    .await
    .map_err(|_err| {
        ErrorResponse::from(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to check duplicate users.",
        )
    })?;
    if same_username_result.is_some() {
        return Err(ErrorResponse::from(
            StatusCode::BAD_REQUEST,
            "Username is already used.",
        ));
    }
    let created_user = sqlx::query!(
        "
            INSERT INTO account (username, display_name, password_hash_and_salt)
            VALUES ($1, $2, $3)
            RETURNING *
        ",
        &req.username,
        req.display_name.as_ref().unwrap_or(&req.username),
        &hash_result
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(|_err| ErrorResponse::from(StatusCode::BAD_REQUEST, "Failed to create user."))?;
    Ok(PublicUser {
        username: created_user.username,
        display_name: created_user.display_name,
    }
    .into())
}

pub async fn post_user(
    AccountId(account_id): AccountId,
    Extension(pg_pool): Extension<PgPool>,
    Json(req): Json<UpdateUserRequest>,
) -> APIResponse<PublicUser> {
    let password_hash_and_salt = req
        .password
        .map(|password| bcrypt::hash(password, BCRYPT_COST).unwrap());

    let updated_user = sqlx::query!(
        "
            UPDATE account
            SET username = COALESCE($2, username),
                display_name = COALESCE($3, display_name),
                password_hash_and_salt = COALESCE($4, password_hash_and_salt)
            WHERE id = $1
            RETURNING *
        ",
        account_id,
        req.username,
        req.display_name,
        password_hash_and_salt
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(|_err| ErrorResponse::from(StatusCode::BAD_REQUEST, "Failed to update user."))?;

    Ok(PublicUser {
        username: updated_user.username,
        display_name: updated_user.display_name,
    }
    .into())
}
