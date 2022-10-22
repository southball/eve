use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

use crate::auth::AccountId;

use super::{APIResponse, ErrorResponse};

#[derive(Serialize)]
pub struct PublicProject {
    id: i64,
    shortcode: String,
    project_name: String,
}

pub async fn get_projects(
    AccountId(account_id): AccountId,
    Extension(pg_pool): Extension<PgPool>,
) -> APIResponse<Vec<PublicProject>> {
    Ok(sqlx::query!(
        "
            SELECT * FROM project
            WHERE account_id = $1
        ",
        account_id
    )
    .fetch_all(&pg_pool)
    .await
    .map_err(|_err| {
        ErrorResponse::from(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch projects.",
        )
    })?
    .into_iter()
    .map(|project| PublicProject {
        id: project.id,
        shortcode: project.shortcode,
        project_name: project.project_name,
    })
    .collect::<Vec<_>>()
    .into())
}

#[derive(Deserialize)]
pub struct CreateOrUpdateProjectRequest {
    shortcode: String,
    project_name: String,
}

pub async fn post_projects(
    AccountId(account_id): AccountId,
    Extension(pg_pool): Extension<PgPool>,
    Json(req): Json<CreateOrUpdateProjectRequest>,
) -> APIResponse<PublicProject> {
    let project = sqlx::query!(
        "
            INSERT INTO project (account_id, shortcode, project_name)
            VALUES ($1, $2, $3)
            RETURNING *
        ",
        account_id,
        &req.shortcode,
        &req.project_name
    )
    .fetch_one(&pg_pool)
    .await
    .map_err(|_err| ErrorResponse::from(StatusCode::BAD_REQUEST, "Failed to create project."))?;

    Ok(PublicProject {
        id: project.id,
        shortcode: project.shortcode,
        project_name: project.project_name,
    }
    .into())
}

pub async fn post_project(
    AccountId(_account_id): AccountId,
    Extension(_pg_pool): Extension<PgPool>,
    Path(_project_id): Path<i64>,
    Json(_req): Json<CreateOrUpdateProjectRequest>,
) -> APIResponse<()> {
    Err(ErrorResponse::from(StatusCode::NOT_FOUND, "Unimplemented"))
}
