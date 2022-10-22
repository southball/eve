mod files;
mod projects;
mod todos;
mod users;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

use files::*;
use projects::*;
use todos::*;
use users::*;

pub fn get_api_router() -> Router {
    Router::new()
        .route("/user", get(get_user).post(post_user))
        .route("/users", post(post_users))
        .route("/login", post(post_login))
        .route("/todos", get(get_todos).post(post_todos))
        .route("/todo/:todo_id", post(post_todo))
        .route(
            "/todo/:todo_id/files",
            get(get_todo_files).post(post_todo_files),
        )
        .route(
            "/todo/:todo_id/file/:file_id",
            get(get_todo_file).post(post_todo_file),
        )
        .route("/files", get(get_files))
        .route("/projects", get(get_projects).post(post_projects))
        .route("/project/:project_id", post(post_project))
}

#[derive(Serialize)]
struct ErrorResponseBody {
    success: bool,
    message: String,
}

pub struct ErrorResponse {
    status_code: StatusCode,
    body: ErrorResponseBody,
}

impl ErrorResponse {
    fn from(status_code: StatusCode, error_message: &str) -> Self {
        ErrorResponse {
            status_code,
            body: ErrorResponseBody {
                success: false,
                message: error_message.to_owned(),
            },
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self.body)).into_response()
    }
}

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    success: bool,
    data: T,
}

impl<T> From<T> for SuccessResponse<T> {
    fn from(data: T) -> Self {
        SuccessResponse {
            success: true,
            data,
        }
    }
}

impl<T> IntoResponse for SuccessResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub type APIResult<T> = Result<T, ErrorResponse>;
pub type APIResponse<T> = Result<SuccessResponse<T>, ErrorResponse>;
