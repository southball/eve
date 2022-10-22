use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
};

use sqlx::postgres::PgPool;

use crate::auth::AccountId;

pub async fn post_todo_files(
    Extension(_pg_pool): Extension<PgPool>,
    Path(_todo_id): Path<String>,
    AccountId(_account_id): AccountId,
) -> impl IntoResponse {
    unimplemented!()
}
