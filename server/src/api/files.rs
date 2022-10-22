use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
};

use sqlx::postgres::PgPool;

use crate::auth::AccountId;

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
